use std::collections::{HashMap, HashSet};

use process_node::Process;

// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
// pub enum NodeInput {
//     Data,
//     Set(UniqueSet),
//     // TODO: Index,
// }

// impl From<UniqueSet> for NodeInput {
//     fn from(set: UniqueSet) -> Self {
//         NodeInput::Set(set)
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub struct TraceNode {
    pub input_sets: HashSet<UniqueSet>,
    pub process: Process,
}

impl TraceNode {
    pub fn are_all_inputs_satisfied(&self, satisfied_inputs: &HashSet<UniqueSet>) -> bool {
        self.input_sets.is_subset(satisfied_inputs)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct UniqueSet {
    id: u32,
}

impl UniqueSet {
    pub fn new(id: u32) -> Self {
        UniqueSet { id }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Trace {
    trace: HashMap<UniqueSet, TraceNode>,
}

impl Trace {
    pub fn new(trace: HashMap<UniqueSet, TraceNode>) -> Self {
        Trace { trace }
    }

    pub fn get_by_output(&self, output: UniqueSet) -> Option<&TraceNode> {
        self.trace.get(&output)
    }
    
    fn output_nodes(&self) -> Vec<(UniqueSet, &TraceNode)> {
        self.trace.iter()
            .filter(|(_, node)| node.process == Process::Output)
            .map(|(output, node)| (*output, node))
            .collect()
    }

    pub fn trace_back_from_outputs<F>(&self, f: &mut F)
    where
        F: FnMut(UniqueSet, &TraceNode),
    {
        fn recurse<F>(output: UniqueSet, node: &TraceNode, trace: &HashMap<UniqueSet, TraceNode>, f: &mut F)
        where
            F: FnMut(UniqueSet, &TraceNode),
        {
            f(output, node);
            for set in &node.input_sets {
                recurse(*set, trace.get(set).unwrap(), trace, f);
            }
        }

        for (output, output_node) in self.output_nodes() {
            recurse(output, output_node, &self.trace, f);
        }
    }
}
