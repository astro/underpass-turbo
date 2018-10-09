use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::HashSet;

use ql::{SetName, StatementSpec, Statement};

#[derive(Debug, PartialEq, Clone)]
pub struct TraceNode {
    inputs: HashSet<Input>,
    statement: Statement,
}

pub fn trace<I>(statement_specs: I) -> HashMap<UniqueSet, TraceNode>
where
    I: Iterator<Item=StatementSpec>,
{
    let mut tracer = Tracer::new();
    for statement_spec in statement_specs {
        trace_statement_spec(statement_spec.clone(), &mut tracer);
    }
    tracer.nodes
}


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct UniqueSet {
    id: u32,
}

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
        UniqueSet {
            id: self.last,
        }
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

    pub fn add_node<'a, I>(&mut self, inputs: I, statement: Statement, output: SetName) -> UniqueSet
    where
        I: Iterator<Item=&'a SetName>,
    {
        let input_sets = HashSet::from_iter(
            inputs
                .filter_map(
                    |name| self.named_sets.get(name)
                        .cloned()
                        .map(Input::Set)
                ));
        self.add_node_inner(input_sets, statement, output)
    }
    
    fn add_node_inner(&mut self, inputs: HashSet<Input>, statement: Statement, output: SetName) -> UniqueSet {
        let output_set = self.unique_set_generator.next();
        self.named_sets.insert(output, output_set);
        self.nodes.insert(output_set, TraceNode {
            inputs,
            statement,
        });
        output_set
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
enum Input {
    DataSource,
    // TODO
    DataIndex,
    /// Set by node id
    Set(UniqueSet),
}

fn trace_statement_spec(statement_spec: StatementSpec, tracer: &mut Tracer) -> UniqueSet {
    let statement = statement_spec.statement;
    let mut inputs = statement_spec.inputs;;
    let output = statement_spec.output;
    let inputs = match &statement {
        &Statement::Union { ref members } =>
            HashSet::from_iter(
                members.iter()
                    .cloned()
                    .map(|member| trace_statement_spec(member, tracer))
                    .map(|output| Input::Set(output))
            ),

        // Statement::Query { filters } => 
        // statement.trace(tracer);
        _ =>
            HashSet::from_iter(
                inputs.iter()
                    .map(|name| tracer.get_set(name))
                    .cloned()
                    .map(Input::Set)
            ),
    };
    tracer.add_node_inner(inputs, statement, output)
}
