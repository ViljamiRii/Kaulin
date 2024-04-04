use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Number,
    Identifier,
    Let,
    Const,
    BinaryOperator,
    Equals,
    Comma,
    Dot,
    Colon,
    SemiColon,
    OpenParen, 
    CloseParen,
    OpenBrace, // {
    CloseBrace, // }
    OpenBracket, // [
    CloseBracket, // ]
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(value: String, token_type: TokenType) -> Self {
        Self { value, token_type }
    }
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

fn is_skippable(c: char) -> bool {
    c.is_whitespace()
}

fn is_int(c: char) -> bool {
    c.is_digit(10)
}

fn get_keywords() -> Vec<(&'static str, TokenType)> {
    vec![
        ("olkoon", TokenType::Let),
        ("vakio", TokenType::Const),
    ]
}


pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source_code.chars().peekable();
    let keywords = get_keywords();

    while let Some(&c) = chars.peek() {
        if c == '(' {
            chars.next();
            tokens.push(Token::new("(".to_string(), TokenType::OpenParen));
        } else if c == ')' {
            chars.next();
            tokens.push(Token::new(")".to_string(), TokenType::CloseParen));
        } else if c == '{' {
            chars.next();
            tokens.push(Token::new("{".to_string(), TokenType::OpenBrace));
        } else if c == '}' {
            chars.next();
            tokens.push(Token::new("}".to_string(), TokenType::CloseBrace));
        } else if c == '[' {
            chars.next();
            tokens.push(Token::new("[".to_string(), TokenType::OpenBracket));
        } else if c == ']' {
            chars.next();
            tokens.push(Token::new("]".to_string(), TokenType::CloseBracket));
        } 
        // Binary operators
        else if c == '+' || c == '-' || c == '*' || c == '/' || c == '%' {
            tokens.push(Token::new(chars.next().unwrap().to_string(), TokenType::BinaryOperator));
        } 
        // Conditional & Assignment tokens
        else if c == '=' {
            chars.next();
            tokens.push(Token::new("=".to_string(), TokenType::Equals));
        } else if c == ';' {
            chars.next();
            tokens.push(Token::new(";".to_string(), TokenType::SemiColon));
        } else if c == ':' {
            chars.next();
            tokens.push(Token::new(":".to_string(), TokenType::Colon));
        } else if c == ',' {
            chars.next();
            tokens.push(Token::new(",".to_string(), TokenType::Comma));
        } else if c == '.' {
            chars.next();
            tokens.push(Token::new(".".to_string(), TokenType::Dot));
        } else if is_int(c) {
            let mut num = String::new();
            while let Some(&c) = chars.peek() {
                if is_int(c) {
                    num.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            tokens.push(Token::new(num, TokenType::Number));
        } else if is_alpha(c) {
            let mut ident = String::new();
            while let Some(&c) = chars.peek() {
                if is_alpha(c) || c.is_digit(10) {
                    ident.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            match keywords.iter().find(|&&(kw, _)| kw == ident.as_str()) {
                Some((_, token_type)) => tokens.push(Token::new(ident.clone(), token_type.clone())),
                None => tokens.push(Token::new(ident, TokenType::Identifier)),
            }
        } else if is_skippable(c) {
            chars.next();
        } else {
            panic!("Unrecognized character found in source: {}", c);
        }
    }

    tokens.push(Token::new("EndOfFile".to_string(), TokenType::EOF));
    tokens
}