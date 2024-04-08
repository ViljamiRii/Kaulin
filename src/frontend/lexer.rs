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
        match c {
            '*' => {
                chars.next();
                if let Some(&next_c) = chars.peek() {
                    if next_c == '*' {
                        chars.next(); // consume the second '*'
                        tokens.push(Token::new("**".to_string(), TokenType::BinaryOperator));
                    } else {
                        tokens.push(Token::new("*".to_string(), TokenType::BinaryOperator));
                    }
                } else {
                    tokens.push(Token::new("*".to_string(), TokenType::BinaryOperator));
                }
            }
            '(' => {
                chars.next();
                tokens.push(Token::new("(".to_string(), TokenType::OpenParen));
            }
            ')' => {
                chars.next();
                tokens.push(Token::new(")".to_string(), TokenType::CloseParen));
            } 
            '{' => {
                chars.next();
                tokens.push(Token::new("{".to_string(), TokenType::OpenBrace));
            } 
            '}' => {
                chars.next();
                tokens.push(Token::new("}".to_string(), TokenType::CloseBrace));
            } 
            '[' => {
                chars.next();
                tokens.push(Token::new("[".to_string(), TokenType::OpenBracket));
            } 
            ']' => {
                chars.next();
                tokens.push(Token::new("]".to_string(), TokenType::CloseBracket));
            }
            '/' => {
                chars.next();
                match chars.peek() {
                    Some(&'/') => {
                        // Single-line comment, consume until newline
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                break;
                            } else {
                                chars.next();
                            }
                        }
                    }
                    Some(&'*') => {
                        // Multi-line comment, consume until '*/'
                        chars.next(); // consume the '*'
                        while let Some(&c) = chars.peek() {
                            if c == '*' {
                                chars.next(); // consume the '*'
                                if let Some(&next_c) = chars.peek() {
                                    if next_c == '/' {
                                        chars.next(); // consume the '/'
                                        break;
                                    }
                                }
                            } else {
                                chars.next();
                            }
                        }
                    }
                    _ => {
                        tokens.push(Token::new("/".to_string(), TokenType::BinaryOperator));
                    }
                }
            }
            '+' | '-' | '*' | '%' | '&' | '|' => {
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
            }
            ';' => {
                chars.next();
                tokens.push(Token::new(";".to_string(), TokenType::SemiColon));
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new("==".to_string(), TokenType::Equal));
                } else {
                    tokens.push(Token::new("=".to_string(), TokenType::Assign));
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new("!=".to_string(), TokenType::NotEqual));
                } else {
                    panic!("Odottamaton '!' lähteessä");
                }
            } 
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new("<=".to_string(), TokenType::LessThanOrEqual));
                } else {
                    tokens.push(Token::new("<".to_string(), TokenType::LessThan));
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(">=".to_string(), TokenType::GreaterThanOrEqual));
                } else {
                    tokens.push(Token::new(">".to_string(), TokenType::GreaterThan));
                }
            }
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(Token::new("&&".to_string(), TokenType::LogicalAnd));
                } else {
                    panic!("Odottamaton '&' lähteessä");
                }
            }
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(Token::new("||".to_string(), TokenType::LogicalOr));
                } else {
                    panic!("Odottamaton '|' lähteessä");
                }
            }
            ':' => {
                chars.next();
                tokens.push(Token::new(":".to_string(), TokenType::Colon));
            }
            ',' => {
                chars.next();
                tokens.push(Token::new(",".to_string(), TokenType::Comma));
            }
            '.' => {
                chars.next();
                tokens.push(Token::new(".".to_string(), TokenType::Dot));
            }
            '"' | '\'' => {
                let quote_type = chars.next().unwrap();
                let mut string_literal = String::new();
                while let Some(&c) = chars.peek() {
                    if c != quote_type {
                        string_literal.push(chars.next().unwrap());
                    } else {
                        chars.next();
                        if !string_literal.is_empty() {
                            tokens.push(Token::new(string_literal.clone(), TokenType::StringLiteral));
                            string_literal.clear();
                        }
                        break;
                    }
                }
                if !string_literal.is_empty() {
                    tokens.push(Token::new(string_literal, TokenType::StringLiteral));
                }
            }
            _ if is_float_or_int(c) => {
                let mut num = String::new();
                let mut has_dot = false;
                while let Some(&c) = chars.peek() {
                    if c == '.' {
                        if has_dot {
                            panic!("Odottamaton '.' numerossa");
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
            }
            _ if is_alpha(c) => {
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
            }
            _ if is_skippable(c) => {
                chars.next();
            }
            _ => {
                panic!("Lähteestä löytyi tuntematon merkki: {}", c);
            }
        }
    }

    tokens.push(Token::new("EndOfFile".to_string(), TokenType::EOF));
    tokens
}