// TODO: delete; use super::tokens::{Token, Tokenizer};
use super::{SetName, StatementSpec, Statement, QueryType, Filter};
use super::syntax::ScriptParser;

pub fn parse(input: &str) -> Vec<StatementSpec> {
    ScriptParser::new()
        .parse(input)
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::super::{SetName, StatementSpec, Statement, RecurseType, Filter, QueryType, TagSpec};
    use super::parse;

    #[test]
    fn test_empty_union() {
        assert_eq!(parse("();"), vec![StatementSpec {
            inputs: vec![],
            statement: Statement::Union { members: vec![] },
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_union() {
        assert_eq!(parse("( node; way; relation; );"), vec![StatementSpec {
            inputs: vec![],
            statement: Statement::Union { members: vec![
                StatementSpec {
                    inputs: vec![],
                    statement: Statement::Query {
                        filters: vec![Filter::QueryType(QueryType::Node)],
                    },
                    output: SetName::default(),
                },
                StatementSpec {
                    inputs: vec![],
                    statement: Statement::Query {
                        filters: vec![Filter::QueryType(QueryType::Way)],
                    },
                    output: SetName::default(),
                },
                StatementSpec {
                    inputs: vec![],
                    statement: Statement::Query {
                        filters: vec![Filter::QueryType(QueryType::Relation)],
                    },
                    output: SetName::default(),
                },
            ] },
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_difference() {
        assert_eq!(parse("( node; - way; );"), vec![StatementSpec {
            inputs: vec![],
            statement: Statement::Difference {
                source: Box::new(StatementSpec {
                    inputs: vec![],
                    statement: Statement::Query {
                        filters: vec![Filter::QueryType(QueryType::Node)],
                    },
                    output: SetName::default(),
                }),
                remove: Box::new(StatementSpec {
                    inputs: vec![],
                    statement: Statement::Query {
                        filters: vec![Filter::QueryType(QueryType::Way)],
                    },
                    output: SetName::default(),
                }),
            },
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_item_default() {
        assert_eq!(parse("._;"), vec![StatementSpec {
            inputs: vec![SetName::default()],
            statement: Statement::Item,
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_item_named_input() {
        assert_eq!(parse(".test;"), vec![StatementSpec {
            inputs: vec![SetName::from("test".to_string())],
            statement: Statement::Item,
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_item_named_input_to_output() {
        assert_eq!(parse(".test -> .new;"), vec![StatementSpec {
            inputs: vec![SetName::from("test".to_string())],
            statement: Statement::Item,
            output: SetName::from("new".to_string()),
        }]);
    }

    #[test]
    fn test_query_node() {
        assert_eq!(parse("node;"), vec![StatementSpec {
            inputs: vec![],
            statement: Statement::Query {
                filters: vec![Filter::QueryType(QueryType::Node)],
            },
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_query_way_filter_id() {
        assert_eq!(parse("way(123);"), vec![StatementSpec {
            inputs: vec![],
            statement: Statement::Query {
                filters: vec![
                    Filter::QueryType(QueryType::Way),
                    Filter::Id(123),
                ],
            },
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_query_way_filter_intersection() {
        assert_eq!(parse("node.a;"), vec![StatementSpec {
            inputs: vec![],
            statement: Statement::Query {
                filters: vec![
                    Filter::QueryType(QueryType::Node),
                    Filter::Intersection(SetName::from("a".to_string())),
                ],
            },
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_query_way_filter_multi_intersection() {
        assert_eq!(parse("node.a.b .c;"), vec![StatementSpec {
            inputs: vec![],
            statement: Statement::Query {
                filters: vec![
                    Filter::QueryType(QueryType::Node),
                    Filter::Intersection(SetName::from("a".to_string())),
                    Filter::Intersection(SetName::from("b".to_string())),
                    Filter::Intersection(SetName::from("c".to_string())),
                ],
            },
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_query_filter_key_exist_string() {
        let expected: Vec<StatementSpec> =
            vec![StatementSpec {
                inputs: vec![],
                statement: Statement::Query {
                    filters: vec![
                        Filter::QueryType(QueryType::Node),
                        Filter::TagExist { k: TagSpec::from_string("name") },
                    ],
                },
                output: SetName::default(),
            }];
        assert_eq!(parse("node[\"name\"];"), expected);
        assert_eq!(parse("node['name'];"), expected);
        assert_eq!(parse("node[name];"), expected);
    }

    #[test]
    fn test_query_filter_key_not_exist_string() {
        let expected: Vec<StatementSpec> =
            vec![StatementSpec {
                inputs: vec![],
                statement: Statement::Query {
                    filters: vec![
                        Filter::QueryType(QueryType::Node),
                        Filter::TagNotExist { k: TagSpec::from_string("name") },
                    ],
                },
                output: SetName::default(),
            }];
        assert_eq!(parse("node[!\"name\"];"), expected);
        assert_eq!(parse("node[! 'name'];"), expected);
        assert_eq!(parse("node[!name];"), expected);
    }

    #[test]
    fn test_query_filter_key_exist_regex() {
        let expected: Vec<StatementSpec> =
            vec![StatementSpec {
                inputs: vec![],
                statement: Statement::Query {
                    filters: vec![
                        Filter::QueryType(QueryType::Node),
                        Filter::TagExist { k: TagSpec::from_regex("name", false) },
                    ],
                },
                output: SetName::default(),
            }];
        assert_eq!(parse("node[~\"name\"];"), expected);
        assert_eq!(parse("node[~'name'];"), expected);
        assert_eq!(parse("node[~name];"), expected);
    }

    #[test]
    fn test_query_filter_key_not_exist_regex() {
        let expected: Vec<StatementSpec> =
            vec![StatementSpec {
                inputs: vec![],
                statement: Statement::Query {
                    filters: vec![
                        Filter::QueryType(QueryType::Node),
                        Filter::TagNotExist { k: TagSpec::from_regex("name", false) },
                    ],
                },
                output: SetName::default(),
            }];
        assert_eq!(parse("node[! ~ \"name\"];"), expected);
        assert_eq!(parse("node[!~'name'];"), expected);
        assert_eq!(parse("node[! ~name];"), expected);
    }

    #[test]
    fn test_query_filter_key_exist_regex_case_insensitive() {
        let expected: Vec<StatementSpec> =
            vec![StatementSpec {
                inputs: vec![],
                statement: Statement::Query {
                    filters: vec![
                        Filter::QueryType(QueryType::Node),
                        Filter::TagExist { k: TagSpec::from_regex("name", true) },
                    ],
                },
                output: SetName::default(),
            }];
        assert_eq!(parse("node[~\"name\",i];"), expected);
        assert_eq!(parse("node[~'name',i];"), expected);
        assert_eq!(parse("node[~name,i];"), expected);
    }

    #[test]
    fn test_query_filter_key_value() {
        let expected: Vec<StatementSpec> =
            vec![StatementSpec {
                inputs: vec![],
                statement: Statement::Query {
                    filters: vec![
                        Filter::QueryType(QueryType::Area),
                        Filter::TagEqual {
                            k: TagSpec::from_string("leisure"),
                            v: TagSpec::from_string("hackerspace"),
                        },
                    ],
                },
                output: SetName::default(),
            }];
        assert_eq!(parse("area[\"leisure\" = \"hackerspace\"];"), expected);
        assert_eq!(parse("area[ 'leisure' = 'hackerspace' ];"), expected);
        assert_eq!(parse("area[leisure=hackerspace];"), expected);
    }

    #[test]
    fn test_query_filter_many() {
        assert_eq!(
            parse(r#"
nwr (5)
    [! ~addr,i]
    (50, 12, 52, 14)
    [internet_access]
    [leisure=hackerspace]
    [amenity=~"workshop",i]
    ;
"#),
            vec![StatementSpec {
                inputs: vec![],
                statement: Statement::Query {
                    filters: vec![
                        Filter::QueryType(QueryType::NWR),
                        Filter::Id(5),
                        Filter::TagNotExist {
                            k: TagSpec::from_regex("addr".to_string(), true)
                        },
                        Filter::BoundingBox {
                            s: 50.0, w: 12.0,
                            n: 52.0, e: 14.0,
                        },
                        Filter::TagExist {
                            k: TagSpec::from_string("internet_access".to_string())
                        },
                        Filter::TagEqual {
                            k: TagSpec::from_string("leisure".to_string()),
                            v: TagSpec::from_string("hackerspace".to_string()),
                        },
                        Filter::TagEqual {
                            k: TagSpec::from_string("amenity".to_string()),
                            v: TagSpec::from_regex("workshop".to_string(), true),
                        },
                    ],
                },
                output: SetName::default(),
            }]
        );
    }

    #[test]
    fn test_recurse() {
        assert_eq!(parse("<; .a <<; > -> .b; .a >> -> .b;"), vec![
            StatementSpec {
                inputs: vec![SetName::default()],
                statement: Statement::Recurse(RecurseType::Up),
                output: SetName::default(),
            },
            StatementSpec {
                inputs: vec![SetName::from("a".to_string())],
                statement: Statement::Recurse(RecurseType::UpRelations),
                output: SetName::default(),
            },
            StatementSpec {
                inputs: vec![SetName::default()],
                statement: Statement::Recurse(RecurseType::Down),
                output: SetName::from("b".to_string()),
            },
            StatementSpec {
                inputs: vec![SetName::from("a".to_string())],
                statement: Statement::Recurse(RecurseType::DownRelations),
                output: SetName::from("b".to_string()),
            },
        ]);
    }

    #[test]
    fn test_output() {
        assert_eq!(parse("out;"), vec![StatementSpec {
            inputs: vec![SetName::default()],
            statement: Statement::Output,
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_output_named_input() {
        assert_eq!(parse(".test out;"), vec![StatementSpec {
            inputs: vec![SetName::from("test".to_string())],
            statement: Statement::Output,
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_output_many_statements() {
        assert_eq!(parse("node->.m; .m->.n; .n out;"), vec![
            StatementSpec {
                inputs: vec![],
                statement: Statement::Query {
                    filters: vec![
                        Filter::QueryType(QueryType::Node),
                    ],
                },
                output: SetName::from("m".to_string()),
            },
            StatementSpec {
                inputs: vec![SetName::from("m".to_string())],
                statement: Statement::Item,
                output: SetName::from("n".to_string()),
            },
            StatementSpec {
                inputs: vec![SetName::from("n".to_string())],
                statement: Statement::Output,
                output: SetName::default(),
            },
        ]);
    }
}
