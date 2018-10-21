use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

use trace_node::{UniqueSet, TraceNode, Trace};
use process_node::{ProcessNode, Process};
use query::QueryTarget;

pub struct Plan {
    outputs: HashMap<UniqueSet, TraceNode>,
    passes: Vec<Vec<UniqueSet>>,
}

impl Plan {
    pub fn run(&self) {
        let mut nodes_by_output = HashMap::new();
        let mut nodes_by_input = HashMap::new();

        for (output, trace_node) in self.outputs.iter() {
            // TODO: processor() instead of clone()
            let process = trace_node.process.clone();
            let process_node = Rc::new(ProcessNode::new(process));
            nodes_by_output.insert(output, process_node.clone());
            for input_set in &trace_node.input_sets {
                match nodes_by_input.entry(input_set.clone()) {
                    Entry::Vacant(mut e) => {
                        e.insert(vec![process_node.clone()]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(process_node.clone());
                    }
                }
            }
        }

        // Connect nodes
        for (output, ref mut process_node) in nodes_by_output.iter_mut() {
            for target in nodes_by_input.get(output).unwrap() {
                Rc::get_mut(process_node)
                    .unwrap()
                    .add_target(target.clone());
            }
        }

        for (pass, _target) in self.passes.iter().enumerate() {
            println!("Running pass {}", pass);
            
        }
    }
}

pub fn plan(trace: &Trace) -> Plan {
    let mut required_outputs = HashSet::new();
    trace.trace_back_from_outputs(&mut |output, trace_node| {
        required_outputs.insert(output);
    });
    println!("required outputs: {:?}", required_outputs);

    // let mut passes = vec![];
    let mut outputs = HashMap::<UniqueSet, TraceNode>::new();
    let mut processed_inputs = HashSet::new();
    let mut passes = vec![];
    while required_outputs.len() > 0 {
        println!("Pass {}", passes.len());
        let mut pass = vec![];

        // For a first step in a pass, all those nodes with data input
        for (set, _) in &outputs {
            processed_inputs.insert(*set);
        }

        required_outputs.retain(|output| {
            let node = trace.get_by_output(*output).unwrap();
            if node.are_all_inputs_satisfied(&processed_inputs) {
                if let Some(target) = node.process.query_target() {
                    println!("Query {:?}: {:?}", output, node.process);
                    outputs.insert(*output, node.clone());
                    // inputs.alter( += node.input_sets
                    pass.push(*output);
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });

        // For latter steps in a pass, append all nodes without data input
        for (set, _) in &outputs {
            processed_inputs.insert(*set);
            
        }

        let mut prev_required_outputs_len = None;
        while prev_required_outputs_len != Some(required_outputs.len()) {
            // println!("prev_required_outputs_len: {:?}", prev_required_outputs_len);
            prev_required_outputs_len = Some(required_outputs.len());
            // println!("required_outputs_len: {:?}", required_outputs.len());

            required_outputs.retain(|output| {
                let node = trace.get_by_output(*output).unwrap();
                if node.are_all_inputs_satisfied(&processed_inputs) {
                    println!("Map {:?}: {:?}", output, node.process);
                    outputs.insert(*output, node.clone());
                    false
                } else {
                    true
                }
            });
            
        }

        // println!("Pass {}: {:?}", pass, pass_outputs);
        passes.push(pass);
        // println!("required_outputs.len: {:?}", required_outputs.len());
    }

    Plan {
        outputs,
        passes,
    }
}

