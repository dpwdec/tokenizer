use regex::Regex;
use std::collections::HashMap;

fn main() {
    tokenize("SELECT SERVICE WHERE port = 443");
}

fn tokenize(input: &str) -> Vec<TokenType> {
    input
        .split_whitespace()
        .map(|raw_element| {
            match attempt_token(raw_element) {
                Some(token) => token,
                None => TokenType::InvalidSyntax
            }
        })
        .collect()
}

fn attempt_token(input: &str) -> Option<TokenType> {
    let regexes: Vec<Regex> = vec![
        Regex::new("^And$").unwrap(),
        Regex::new("[a-z0-9]+").unwrap(),
        Regex::new("^CONTAINS$").unwrap(),
        Regex::new("^=$").unwrap(),
        Regex::new("^SELECT$").unwrap(),
        Regex::new("^WHERE$").unwrap(),
        Regex::new("^ROUTE$|SERVICE|ALL|GLOBAL$").unwrap(),
    ];

    match regexes
        .iter()
        .find(|regex| regex.is_match(input))
        .map(|regex| regex.as_str())
    {
        Some("^AND$") => Some(TokenType::And),
        Some("[a-z0-9]+") => Some(TokenType::StringValue(String::from(input))),
        Some("^CONTAINS$") => Some(TokenType::Contains),
        Some("^=$") => Some(TokenType::Equals),
        Some("^SELECT$") => Some(TokenType::Select),
        Some("^WHERE$") => Some(TokenType::Where),
        Some("^ROUTE$|SERVICE|ALL|GLOBAL$") => {
            match input {
                "GLOBAL" => Some(TokenType::Level(ConfigLevel::Global)),
                "SERVICE" => Some(TokenType::Level(ConfigLevel::Service)),
                "ROUTE" => Some(TokenType::Level(ConfigLevel::Route)),
                _ => Some(TokenType::Level(ConfigLevel::All))
            }
        },
        _ => None
    }
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Level(ConfigLevel),
    StringValue(String),
    And,
    Contains,
    Equals,
    Select,
    Where,
    InvalidSyntax
}

#[derive(Debug, PartialEq)]
enum ConfigLevel {
    Service,
    Route,
    Global,
    All
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let tokens = tokenize("SELECT SERVICE WHERE port = 443");
        let exp_tokens = vec![
            TokenType::Select,
            TokenType::Level(ConfigLevel::Service),
            TokenType::Where,
            TokenType::StringValue(String::from("port")),
            TokenType::Equals,
            TokenType::StringValue(String::from("443"))
        ];
        assert_eq!(tokens, exp_tokens)
    }

    #[test]
    fn test_attempt_token() {
        match attempt_token("443") {
            Some(token) => println!("{:?}", token),
            None => println!("No matching token")
        }
    }
}