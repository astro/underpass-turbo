use regex::{Regex, RegexBuilder};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetName(String);

impl Default for SetName {
    fn default() -> Self {
        SetName("_".to_owned())
    }
}

#[derive(Debug)]
pub struct StatementSpec {
    inputs: Vec<SetName>,
    statement: Statement,
    output: SetName,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Query {
        filters: Vec<Filter>,
    },
    Recurse,
    IsInArea,
    Union {
        members: Vec</*Box<*/Statement/*>*/>,
    },
    /// Source from a set
    Item,
    /// Output
    Print,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum QueryType {
    Node,
    Way,
    Relation,
    Derived,
    Area,
    /// Node+Way+Relation
    NWR,
}

#[derive(Debug, Clone)]
pub enum Filter {
    QueryType(QueryType),
    Id(u64),
    BoundingBox {
        s: f64,
        w: f64,
        n: f64,
        e: f64
    },
    TagEqual {
        k: TagSpec,
        v: TagSpec,
    },
    TagNotEqual {
        k: TagSpec,
        v: TagSpec,
    },
    TagExist {
        k: TagSpec,
    },
    TagNotExist {
        k: TagSpec,
    },
    // Recurse {
    //     recurse_target: (),
    //     input: SetName,
    // },
}

#[derive(Debug, Clone)]
pub enum TagSpec {
    String(String),
    Regex(Regex),
}

impl TagSpec {
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        TagSpec::String(s.into())
    }

    pub fn from_regex(r: &str, case_insensitive: bool) -> Self {
        let regex = RegexBuilder::new(r)
            .case_insensitive(case_insensitive)
            .multi_line(true)
            .ignore_whitespace(true)
            .unicode(true)
            .build().unwrap();
        TagSpec::Regex(regex)
    }

    pub fn test(&self, s: &str) -> bool {
        match self {
            &TagSpec::Regex(ref r) =>
                r.is_match(s),
            &TagSpec::String(ref ss) =>
                s == ss,
        }
    }
}
