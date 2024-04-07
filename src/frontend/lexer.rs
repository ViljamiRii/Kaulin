use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Integer,
    Float,
    Identifier,
    StringLiteral,

    PlusEqual, // +=
    MinusEqual, // -=
    Equal, // ==
    NotEqual, // !=
    LessThan, // <
    GreaterThan, // >
    LessThanOrEqual, // <=
    GreaterThanOrEqual, // >=
    SingleLineComment, // //
    MultiLineComment, // /* */
    LogicalAnd,
    LogicalOr,
    Let,
    Const,
    Fn,
    If,
    Else,
    While,
    For,
    Absolute,
    BinaryOperator,
    Assign,
    Comma,
    Dot,
    Exponent,
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
    c.is_alphabetic() || c == '_'
}

fn is_skippable(c: char) -> bool {
    c.is_whitespace()
}

fn is_float_or_int(c: char) -> bool {
    c.is_digit(10) || c == '.'
}

fn get_keywords() -> Vec<(&'static str, TokenType)> {
    vec![
        ("olkoon", TokenType::Let), 
        ("vakio", TokenType::Const), 
        ("funktio", TokenType::Fn),
        ("jos", TokenType::If),
        ("muuten", TokenType::Else),
        ("kun", TokenType::While),
        ("toista", TokenType::For),
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
        } else if c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '&' || c == '|' {
            if c == '/' {
                if let Some(&next_c) = chars.peek() {
                    if next_c == '/' {
                        // Single-line comment
                        chars.next(); // consume the second '/'
                        while let Some(&c) = chars.peek() {
                            if c != '\n' {
                                chars.next(); // consume the comment character
                            } else {
                                break;
                            }
                        }
                        continue; // skip to the next character
                    } else if next_c == '*' {
                        // Multi-line comment
                        chars.next(); // consume the '*'
                        let mut prev_c = ' ';
                        while let Some(&c) = chars.peek() {
                            if prev_c == '*' && c == '/' {
                                chars.next(); // consume the '/'
                                break;
                            } else {
                                prev_c = chars.next().unwrap();
                            }
                        }
                        continue; // skip to the next character
                    }
                }
            }
        
            let mut operator = chars.next().unwrap().to_string();
            if (c == '&' && chars.peek() == Some(&'&')) || (c == '|' && chars.peek() == Some(&'|')) {
                operator.push(chars.next().unwrap());
            } else if (c == '+' || c == '-') && chars.peek() == Some(&'=') {
                operator.push(chars.next().unwrap());
            }
        
            let token_type = match operator.as_str() {
                "+=" => TokenType::PlusEqual,
                "-=" => TokenType::MinusEqual,
                _ => TokenType::BinaryOperator,
            };
            tokens.push(Token::new(operator, token_type));
        } else if c == ';' {
            chars.next();
            tokens.push(Token::new(";".to_string(), TokenType::SemiColon));
        } else if c == '=' {
            chars.next();
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::new("==".to_string(), TokenType::Equal));
            } else {
                tokens.push(Token::new("=".to_string(), TokenType::Assign));
            }
        } else if c == '!' {
            chars.next();
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::new("!=".to_string(), TokenType::NotEqual));
            } else {
                panic!("Unexpected '!' in source");
            }
        } else if c == '<' {
            chars.next();
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::new("<=".to_string(), TokenType::LessThanOrEqual));
            } else {
                tokens.push(Token::new("<".to_string(), TokenType::LessThan));
            }
        } else if c == '>' {
            chars.next();
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::new(">=".to_string(), TokenType::GreaterThanOrEqual));
            } else {
                tokens.push(Token::new(">".to_string(), TokenType::GreaterThan));
            }
        } else if c == '&' {
            chars.next();
            if chars.peek() == Some(&'&') {
                chars.next();
                tokens.push(Token::new("&&".to_string(), TokenType::LogicalAnd));
            } else {
                panic!("Unexpected '&' in source");
            }
        } else if c == '|' {
            chars.next();
            if chars.peek() == Some(&'|') {
                chars.next();
                tokens.push(Token::new("||".to_string(), TokenType::LogicalOr));
            } else {
                panic!("Unexpected '|' in source");
            }
        } else if c == ':' {
            chars.next();
            tokens.push(Token::new(":".to_string(), TokenType::Colon));
        } else if c == ',' {
            chars.next();
            tokens.push(Token::new(",".to_string(), TokenType::Comma));
        } else if c == '.' {
            chars.next();
            tokens.push(Token::new(".".to_string(), TokenType::Dot));
        } else if c == '"' || c == '\'' {
            let quote_type = chars.next().unwrap();
            let mut string_literal = String::new();
            while let Some(&c) = chars.peek() {
                if c != quote_type {
                    string_literal.push(chars.next().unwrap());
                } else {
                    chars.next();
                    break;
                }
            }
            tokens.push(Token::new(string_literal, TokenType::StringLiteral));
        } else if is_float_or_int(c) {
            let mut num = String::new();
            let mut has_dot = false;
            while let Some(&c) = chars.peek() {
                if c == '.' {
                    if has_dot {
                        panic!("Unexpected '.' in number");
                    } else {
                        has_dot = true;
                        num.push(chars.next().unwrap());
                    }
                } else if c.is_digit(10) {
                    num.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if has_dot {
                tokens.push(Token::new(num, TokenType::Float));
            } else {
                tokens.push(Token::new(num, TokenType::Integer));
            }
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

    //println!("{:?}", tokens);
    tokens.push(Token::new("EndOfFile".to_string(), TokenType::EOF));
    tokens
}
