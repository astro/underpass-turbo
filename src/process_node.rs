use ql::{Filter, RecurseType};
use trace::UniqueSet;


#[derive(Debug, PartialEq, Clone)]
pub enum ProcessNode {
    Query {
        filters: Vec<Filter>,
    },
    Difference {
        source: UniqueSet,
        remove: UniqueSet,
    },
    Union,
    Recurse(RecurseType),
    Output,
}
