use super::tokens::{Token, Tokenizer};
use super::{SetName, StatementSpec, Statement, QueryType, Filter};


pub fn parse(input: &str) -> Vec<StatementSpec> {
    let all_tokens = Tokenizer::new(input)
        .collect::<Vec<_>>();
    let mut tokens = &all_tokens[..];
    let mut results = vec![];
    while tokens.len() > 0 {
        let m = parse_statement(tokens);
        let statement_spec = m.0;
        tokens = m.1;
        results.push(statement_spec);
    }
    results
}

fn parse_statement(mut tokens: &[(usize, Token)]) -> (StatementSpec, &[(usize, Token)]) {
    let mut inputs = vec![];
    let first_token = tokens.get(0).map(|t| &t.1);
    let statement = if first_token == Some(&Token::ParenthesisOpen) {
        tokens = &tokens[1..];
        let mut members = vec![];
        while tokens.get(0).map(|t| &t.1) != Some(&Token::ParenthesisClose) {
            let m = parse_statement(tokens);
            let statement_spec = m.0;
            members.push(statement_spec);
            tokens = m.1;
            if tokens.len() < 1 {
                panic!("Unterminated union");
            }
        }
        tokens = &tokens[1..];
        Statement::Union { members }
    } else if first_token == Some(&Token::Period) {
        // All statements that take an input set
        match tokens.get(1) {
            Some((_, Token::StringLiteral(ref input))) => {
                inputs.push(SetName::from(input.to_string()));
                tokens = &tokens[2..];
                Statement::Item
            }
            Some((ref pos, ref token)) =>
                panic!("Expected string literal at char {}, found: {:?}", pos, token),
            None =>
                panic!("Unexpected end of input after token '.'"),
        }
    } else if let Some(Token::StringLiteral(literal)) = first_token {
        // All statements that begin with a string literal
        if let Some(qt) = QUERY_TYPES.iter().find(|(ref name, _)| literal == name) {
            tokens = &tokens[1..];
            let mut filters = vec![
                Filter::QueryType(qt.1),
            ];
            Statement::Query {
                filters
            }
        } else {
            panic!("Cannot parse literal {:?}", literal);
        }
    } else if let Some((pos, token)) = tokens.get(0) {
        panic!("Unexpected token at char {}: {:?}", pos, token);
    } else {
        panic!("Unexpected end of input");
    };

    let output = match tokens.get(0..3) {
        Some([(_, Token::Arrow), (_, Token::Period), (_, Token::StringLiteral(ref output))]) => {
            tokens = &tokens[3..];
            SetName::from(output.to_string())
        }
        _ => SetName::default(),
    };

    match tokens.get(0) {
        Some((_, Token::Semicolon)) =>
            // ;
            (),
        Some((pos, token)) =>
            panic!("Expected semicolon at char {}, found: {:?}", pos, token),
        None =>
            panic!("Unexpected end of input when expecting ';'"),
    }
    tokens = &tokens[1..];

    let statement_spec = StatementSpec {
        inputs,
        statement,
        output,
    };
    (statement_spec, tokens)
}

const QUERY_TYPES: &[(&'static str, QueryType)] = &[
    ("node", QueryType::Node),
    ("way", QueryType::Way),
    ("relation", QueryType::Relation),
    ("derived", QueryType::Derived),
    ("area", QueryType::Area),
    ("nwr", QueryType::NWR),
];


#[cfg(test)]
mod tests {
    use super::super::{SetName, StatementSpec, Statement, Filter, QueryType};
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
    fn test_item_default() {
        assert_eq!(parse("._;"), vec![StatementSpec {
            inputs: vec![SetName::default()],
            statement: Statement::Item,
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_item_named_source() {
        assert_eq!(parse(".test;"), vec![StatementSpec {
            inputs: vec![SetName::from("test".to_string())],
            statement: Statement::Item,
            output: SetName::default(),
        }]);
    }

    #[test]
    fn test_item_named_source_to_dest() {
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
}
