use regex::Regex;
use std::collections::HashMap;

fn main() {
    let tokens = tokenize("SELECT SERVICE WHERE port = 443");
    if tokens.iter().any(|token| match token { TokenType::InvalidSyntax(_) => true, _ => false }) {
        tokens
            .iter()
            .enumerate()
            .for_each(|(index, invalid)| {
                match invalid {
                    TokenType::InvalidSyntax(syntax) => {
                        println!("Invalid syntax \"{}\" at token position {}.", syntax, index)
                    },
                    _ => ()
                }
            })
    } else {
        println!("{:?}", tokens)
    }
}

fn tokenize(input: &str) -> Vec<TokenType> {
    input
        .split_whitespace()
        .map(|raw_element| attempt_token(raw_element))
        .collect()
}

fn attempt_token(input: &str) -> TokenType {
    let mut tokens: HashMap<&str, TokenType> = HashMap::new();
    tokens.insert("^SELECT$", TokenType::Select);
    tokens.insert("^AND$", TokenType::And);
    tokens.insert("^CONTAINS$", TokenType::Contains);
    tokens.insert("^WHERE$", TokenType::Where);
    tokens.insert("^ROUTE$", TokenType::Level(ConfigLevel::Route));
    tokens.insert("^SERVICE$", TokenType::Level(ConfigLevel::Service));
    tokens.insert("^ALL$", TokenType::Level(ConfigLevel::All));
    tokens.insert("^GLOBAL$", TokenType::Level(ConfigLevel::Global));
    tokens.insert("^=$", TokenType::Equals);
    tokens.insert("^[a-z0-9]+$", TokenType::StringValue(String::new()));

    let regexes: Vec<Regex> = tokens.keys().map(|regex| Regex::new(regex).unwrap()).collect();

    let token_key: &str = match regexes.iter().find(|regex| regex.is_match(input)) {
        Some(regex) => regex.as_str(),
        None => ""
    };

    match tokens.get(token_key) {
        Some(TokenType::StringValue(_)) => TokenType::StringValue(String::from(input)),
        Some(token) => token.clone(),
        None => TokenType::InvalidSyntax(String::from(input))
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Level(ConfigLevel),
    StringValue(String),
    And,
    Contains,
    Equals,
    Select,
    Where,
    InvalidSyntax(String)
}

#[derive(Debug, PartialEq, Clone)]
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
        let tokens = tokenize("SELECT SERVICE WHERE port = 443 AND protocol CONTAINS get post");
        let exp_tokens = vec![
            TokenType::Select,
            TokenType::Level(ConfigLevel::Service),
            TokenType::Where,
            TokenType::StringValue(String::from("port")),
            TokenType::Equals,
            TokenType::StringValue(String::from("443")),
            TokenType::And,
            TokenType::StringValue(String::from("protocol")),
            TokenType::Contains,
            TokenType::StringValue(String::from("get")),
            TokenType::StringValue(String::from("post"))
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