use std::rc::Rc;
use std::sync::Arc;
use std::collections::HashMap;

use ql::{Filter, RecurseType};
use trace_node::UniqueSet;
use set::Set;
use item::Item;
use query::QueryTarget;

#[derive(Debug, PartialEq, Clone)]
pub struct ProcessNode {
    process: Process,
    targets: Vec<Rc<ProcessNode>>,
}

impl ProcessNode {
    pub fn new(process: Process) -> Self {
        ProcessNode {
            process,
            targets: vec![],
        }
    }

    pub fn add_target(&mut self, target: Rc<ProcessNode>) {
        self.targets.push(target);
    }

    pub fn targets(&self) -> &[Rc<ProcessNode>] {
        &self.targets[..]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Process {
    /// Data query
    Query {
        filters: Vec<Filter>,
    },
    /// Must buffer
    Difference {
        source: UniqueSet,
        remove: UniqueSet,
    },
    /// Pass-through
    Union,
    /// Data query, with optional index
    Recurse(RecurseType),
    /// Streaming output
    Output,
}

impl Process {
    pub fn query_target(&self) -> Option<QueryTarget> {
        match self {
            Process::Query { filters } => {
                let filters = Arc::new(filters.clone());
                Some(QueryTarget::Query { filters })
            }
            Process::Recurse(rt) =>
                Some(QueryTarget::Recurse(*rt)),
            _ =>
                None,
        }
    }

    pub fn process(&mut self, input_set: UniqueSet, item: Item) -> Set {
        // TODO
        Set::empty()
    }
}
