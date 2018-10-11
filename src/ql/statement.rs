use regex::{Regex, RegexBuilder};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SetName(String);

impl From<String> for SetName {
    fn from(s: String) -> Self {
        SetName(s)
    }
}

impl Default for SetName {
    fn default() -> Self {
        SetName("_".to_owned())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementSpec {
    pub inputs: Vec<SetName>,
    pub statement: Statement,
    pub output: SetName,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Query {
        filters: Vec<Filter>,
    },
    Recurse(RecurseType),
    IsInArea,
    Union {
        members: Vec<StatementSpec>,
    },
    Difference {
        source: Box<StatementSpec>,
        remove: Box<StatementSpec>,
    },
    /// Source from a set
    Item,
    Output,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum RecurseType {
    Up,
    UpRelations,
    Down,
    DownRelations,
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

#[derive(Debug, PartialEq, Clone)]
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
    Intersection(SetName),
    // Recurse {
    //     recurse_target: (),
    //     input: SetName,
    // },
}

#[derive(Debug, Clone)]
pub enum TagSpec {
    String(String),
    Regex(String, bool, Regex),
}

impl TagSpec {
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        TagSpec::String(s.into())
    }

    pub fn from_regex<S: Into<String>>(r: S, case_insensitive: bool) -> Self {
        let s = r.into();
        let regex = RegexBuilder::new(&s)
            .case_insensitive(case_insensitive)
            .multi_line(true)
            .ignore_whitespace(true)
            .unicode(true)
            .build().unwrap();
        TagSpec::Regex(s, case_insensitive, regex)
    }

    pub fn test(&self, s: &str) -> bool {
        match self {
            &TagSpec::Regex(_, _, ref r) =>
                r.is_match(s),
            &TagSpec::String(ref ss) =>
                s == ss,
        }
    }
}

impl PartialEq for TagSpec {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TagSpec::String(s1), TagSpec::String(s2)) =>
                s1 == s2,
            (TagSpec::Regex(s1, ci1, _), TagSpec::Regex(s2, ci2, _)) =>
                ci1 == ci2 &&
                s1 == s2,
            _ =>
                false,
        }
    }
}
