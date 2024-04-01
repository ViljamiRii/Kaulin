use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Number,
    Identifier,
    Let,
    BinaryOperator,
    Equals,
    OpenParen,
    CloseParen,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(value: String, token_type: TokenType) -> Self {
        Self { value, token_type }
    }
}

pub fn get_keywords() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("olkoon", TokenType::Let);
    keywords
}

pub fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

pub fn is_skippable(c: char) -> bool {
    c.is_whitespace()
}

pub fn is_int(c: char) -> bool {
    c.is_numeric()
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source_code.chars().collect::<Vec<char>>();
    let keywords = get_keywords();

    while !chars.is_empty() {
        if chars[0] == '(' {
            tokens.push(Token::new(chars.remove(0).to_string(), TokenType::OpenParen));
        } else if chars[0] == ')' {
            tokens.push(Token::new(chars.remove(0).to_string(), TokenType::CloseParen));
        } else if "+-*/%".contains(chars[0]) {
            tokens.push(Token::new(chars.remove(0).to_string(), TokenType::BinaryOperator));
        } else if chars[0] == '=' {
            tokens.push(Token::new(chars.remove(0).to_string(), TokenType::Equals));
        } else if is_int(chars[0]) {
            let mut num = String::new();
            while !chars.is_empty() && is_int(chars[0]) {
                num.push(chars.remove(0));
            }
            tokens.push(Token::new(num, TokenType::Number));
        } else if is_alpha(chars[0]) {
            let mut ident = String::new();
            while !chars.is_empty() && is_alpha(chars[0]) {
                ident.push(chars.remove(0));
            }
            match keywords.get(ident.as_str()) {
                Some(&token_type) => tokens.push(Token::new(ident, token_type)),
                None => tokens.push(Token::new(ident, TokenType::Identifier)),
            }
        } else if is_skippable(chars[0]) {
            chars.remove(0);
        } else {
            panic!("Unrecognized character found in source: {}", chars[0]);
        }
    }

    tokens.push(Token::new("EndOfFile".to_string(), TokenType::EOF));
    tokens
}