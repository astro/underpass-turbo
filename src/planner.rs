use std::collections::{HashMap, HashSet};

use trace::{UniqueSet, TraceNode};
use process_node::ProcessNode;

pub fn plan(trace: &HashMap<UniqueSet, TraceNode>) {
    let mut required_outputs = HashSet::new();
    trace_back_from_outputs(trace, &mut |output, trace_node| {
        for set in &trace_node.input_sets {
            required_outputs.insert(*set);
        }
    });
    println!("required outputs: {:?}", required_outputs);
}

fn trace_back_from_outputs<F>(trace: &HashMap<UniqueSet, TraceNode>, f: &mut F)
where
    F: FnMut(UniqueSet, &TraceNode),
{
    fn recurse<F>(output: UniqueSet, node: &TraceNode, trace: &HashMap<UniqueSet, TraceNode>, f: &mut F)
    where
        F: FnMut(UniqueSet, &TraceNode),
    {
        f(output, node);
        for set in &node.input_sets {
            recurse(output, trace.get(set).unwrap(), trace, f);
        }
    }

    let output_nodes = trace.iter()
        .filter(|(_, node)| node.process_node == ProcessNode::Output)
        .collect::<Vec<_>>();
    for (output, ref output_node) in output_nodes {
        recurse(*output, output_node, trace, f);
    }
}
