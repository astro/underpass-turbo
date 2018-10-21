use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::HashSet;

use ql::{SetName, StatementSpec, Statement};
use trace_node::{Trace, TraceNode, UniqueSet};
use process_node::Process;

/// Execute a query script to establish a graph representation of the
/// data flow.
pub fn trace<I>(statement_specs: I) -> Trace
where
    I: Iterator<Item=StatementSpec>,
{
    let mut tracer = Tracer::new();
    for statement_spec in statement_specs {
        trace_statement_spec(statement_spec, &mut tracer);
    }
    Trace::new(tracer.nodes)
}

/// Because sets can be overwritten by name, we assign unique ids to
/// each output in the data flow graph.
struct UniqueSetGenerator {
    last: u32,
}

impl UniqueSetGenerator {
    fn new() -> Self {
        UniqueSetGenerator {
            last: 0,
        }
    }

    fn next(&mut self) -> UniqueSet {
        self.last += 1;
        UniqueSet::new(self.last)
    }
}

pub struct Tracer {
    // by name while tracing
    named_sets: HashMap<SetName, UniqueSet>,
    unique_set_generator: UniqueSetGenerator,
    // result by output set
    nodes: HashMap<UniqueSet, TraceNode>,
}

impl Tracer {
    fn new() -> Self {
        let named_sets = HashMap::new();
        let unique_set_generator = UniqueSetGenerator::new();
        let nodes = HashMap::new();
        Tracer {
            named_sets,
            unique_set_generator,
            nodes,
        }
    }

    pub fn get_set(&self, name: &SetName) -> &UniqueSet {
        self.named_sets.get(name)
            .expect(&format!("No such set named {:?}", name))
    }

    pub fn link(&mut self, input_set: UniqueSet, output_set: SetName) {
        self.named_sets.insert(output_set, input_set);
    }

    pub fn add_node<'a, I>(&mut self, inputs: I, process: Process, output: SetName) -> UniqueSet
    where
        I: Iterator<Item=&'a SetName>,
    {
        let input_sets = inputs.map(
            |name| self.get_set(name).clone()
        ).collect();
        self.add_node_with_sets(input_sets, process, output)
    }

    fn add_node_with_sets(&mut self, input_sets: HashSet<UniqueSet>, process: Process, output: SetName) -> UniqueSet {
        let output_set = self.unique_set_generator.next();
        self.named_sets.insert(output, output_set);

        self.nodes.insert(output_set, TraceNode {
            input_sets,
            process,
        });

        output_set
    }
}

/// Returns output set
fn trace_statement_spec(statement_spec: StatementSpec, tracer: &mut Tracer) -> UniqueSet {
    let statement = statement_spec.statement;
    let statement_inputs = statement_spec.inputs;
    let output = statement_spec.output;
    match statement {
        Statement::Union { members } => {
            let input_sets = members.into_iter()
                .map(|member|
                     trace_statement_spec(member, tracer)
                ).collect();
            let node = Process::Union;
            tracer.add_node_with_sets(input_sets, node, output)
        }
        Statement::Difference { source, remove } => {
            let source_input =
                trace_statement_spec(*source, tracer);
            let remove_input =
                trace_statement_spec(*remove, tracer);
            let node = Process::Difference {
                source: source_input,
                remove: remove_input,
            };
            tracer.add_node_with_sets(
                HashSet::from_iter([
                    source_input,
                    remove_input,
                ].iter().cloned()),
                node, output)
        }
        Statement::Query { filters } => {
            let node = Process::Query { filters };
            let input_sets = HashSet::new();
            tracer.add_node_with_sets(input_sets, node, output)
        }
        Statement::Recurse(rt) => {
            let node = Process::Recurse(rt);
            let input_sets = statement_inputs.iter()
                .map(|name| tracer.get_set(name))
                .cloned()
                .collect::<HashSet<_>>();
            tracer.add_node_with_sets(input_sets, node, output)
        }
        Statement::Item => {
            if statement_inputs.len() != 1 {
                panic!("item statement with less/more than one input set");
            }
            let input_set = tracer.get_set(&statement_inputs[0])
                .clone();
            // Needs no node representation in the flow graph
            tracer.link(input_set.clone(), output);
            input_set
        }
        Statement::Output => {
            let node = Process::Output;
            tracer.add_node(statement_inputs.iter(), node, output)
        }
        _ =>
            panic!("Not implemented: {:?}", statement),
    }
}


#[cfg(test)]
mod tests {
    use super::{Input, SetName, StatementSpec, Statement, Process};
    use super::{trace, TraceNode};

    #[test]
    fn test_trace_simple() {
        let nodes = trace([
            StatementSpec {
                inputs: vec![],
                statement: Statement::Query { filters: vec![] },
                output: SetName::default(),
            },
            StatementSpec {
                inputs: vec![SetName::default()],
                statement: Statement::Output,
                output: SetName::default(),
            },
        ].into_iter().cloned());
        let output_nodes = nodes.iter()
            .filter(|(_, node)| node.process == Process::Output)
            .collect::<Vec<_>>();
        assert_eq!(output_nodes.len(), 1);
        let output_inputs = &output_nodes[0].1.inputs;
        let query_nodes = nodes.iter()
            .filter(|(output, _)| output_inputs.contains(*output))
            .collect::<Vec<_>>();
        assert_eq!(query_nodes.len(), 1);
        assert_eq!(query_nodes[0].1.process, Process::Query { filters: vec![] });
    }
}
