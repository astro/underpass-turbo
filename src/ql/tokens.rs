#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    ParenthesisOpen,
    ParenthesisClose,
    BracketOpen,
    BracketClose,
    StringLiteral(String),
    Number(f64),
    Period,
    Arrow,
    DoubleColon,
    Colon,
    Semicolon,
    Difference,
    RecurseUpRelations,
    RecurseUp,
    RecurseDownRelations,
    RecurseDown,
    Equals,
    EqualsNot,
    Matches,
    MatchesNot,
    Not,
}

const LITERAL_TOKENS: &'static [(&'static str, Token)] = &[
    ("(", Token::ParenthesisOpen),
    (")", Token::ParenthesisClose),
    ("[", Token::BracketOpen),
    ("]", Token::BracketClose),
    (".", Token::Period),
    ("->", Token::Arrow),
    ("::", Token::DoubleColon),
    (":", Token::Colon),
    (";", Token::Semicolon),
    ("-", Token::Difference),
    ("<<", Token::RecurseUpRelations),
    ("<", Token::RecurseUp),
    (">>", Token::RecurseDownRelations),
    (">", Token::RecurseDown),
    ("=", Token::Equals),
    ("!=", Token::EqualsNot),
    ("~", Token::Matches),
    ("!~", Token::MatchesNot),
    ("!", Token::Not),
];

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
        }
    }
}

impl Iterator for Tokenizer {
    type Item = (usize, Token);
    
    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace
        while self.position < self.input.len() &&
            self.input[self.position].is_whitespace() {
                self.position += 1;
            }

        if self.position >= self.input.len() {
            // EOF
            return None
        }

        for &(ref s, ref token) in LITERAL_TOKENS {
            if input_starts_with(&self.input[self.position..], s) {
                let result = (self.position, token.clone());
                self.position += s.len();
                return Some(result);
            }
        }

        let c = self.input[self.position];
        if c == '"' || c == '\'' {
            return parse_string_literal(c, &self.input, self.position + 1)
                .map(|(position, s)| {
                    let result = (self.position, Token::StringLiteral(s));
                    self.position = position;
                    result
                });
        }

        match parse_number(&self.input, self.position) {
            Some((position, n)) => {
                let result = (self.position, Token::Number(n));
                self.position = position;
                return Some(result);
            },
            _ => (),
        }

        match parse_bare_literal(&self.input, self.position) {
            Some((position, s)) => {
                let result = (self.position, Token::StringLiteral(s));
                self.position = position;
                return Some(result);
            },
            _ => (),
        }

        None
    }
}

fn input_starts_with(input: &[char], other: &str) -> bool {
    for (i, o) in other.char_indices() {
        if input[i] != o {
            return false
        }
    }

    true
}

fn parse_number(input: &[char], mut position: usize) -> Option<(usize, f64)> {
    if ! input[position].is_digit(10) {
        return None
    }

    let mut result = String::new();
    while position < input.len() && input[position].is_digit(10) {
        result.push(input[position]);
        position += 1;
    }

    if position < input.len() && input[position] == '.' {
        result.push('.');
        position += 1;
        while position < input.len() && input[position].is_digit(10) {
            result.push(input[position]);
            position += 1;
        }
    }

    match result.parse() {
        Ok(n) => Some((position, n)),
        _ => None
    }
}

fn parse_bare_literal(input: &[char], mut position: usize) -> Option<(usize, String)> {
    let mut result = String::new();

    let mut c = input[position];
    while c.is_alphabetic() || c.is_digit(10) || c == '_' {
        result.push(c);
        
        position += 1;
        c = input[position];
    }

    if result.len() > 0 {
        Some((position, result))
    } else {
        None
    }
}

fn parse_string_literal(delim: char, input: &[char], mut position: usize) -> Option<(usize, String)> {
    let mut result = String::new();
    while position < input.len() {
        let c = input[position];

        if c == delim {
            return Some((position + 1, result));
        } else if c == '\\' && position + 1 < input.len() {
            position += 1;
            let c = input[position];
            result.push(c);
        } else {
            result.push(c);
        }
        position += 1;
    }

    // Runaway literal
    None
}

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};

    fn tokenize(input: &str) -> Vec<Token> {
        Tokenizer::new(input)
            .map(|(_position, token)| token)
            .collect()
    }
    
    #[test]
    fn numbers() {
        assert_eq!(tokenize("23.5 . 5 . 6 . 7.8"),
                   vec![
                       Token::Number(23.5f64),
                       Token::Period,
                       Token::Number(5f64),
                       Token::Period,
                       Token::Number(6f64),
                       Token::Period,
                       Token::Number(7.8f64),
                   ]);
    }
    
    #[test]
    fn string_literals() {
        assert_eq!(tokenize("\"foo\" 'bar' \"quux\\\"fnord\""),
                   vec![
                       Token::StringLiteral("foo".to_owned()),
                       Token::StringLiteral("bar".to_owned()),
                       Token::StringLiteral("quux\"fnord".to_owned()),
                   ]);
    }
    
    #[test]
    fn it_works() {
        assert_eq!(tokenize("(._; >;);"),
                   vec![Token::ParenthesisOpen,
                        Token::Period,
                        Token::StringLiteral("_".to_owned()),
                        Token::Semicolon,
                        Token::RecurseDown,
                        Token::Semicolon,
                        Token::ParenthesisClose,
                        Token::Semicolon
                   ]);
    }
}
