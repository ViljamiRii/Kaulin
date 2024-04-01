use crate::frontend::lexer::{Token, TokenType, tokenize};
use crate::frontend::ast::{Expr, Stmt, Program, BinaryExpr, Identifier, NumericLiteral, ExprStmt};

// Parser struct holds the tokens to be parsed and the current position in the tokens vector
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // new function creates a new Parser with an empty tokens vector and current set to 0
    pub fn new() -> Self {
        Self { tokens: Vec::new(), current: 0 }
    }

    // not_eof checks if the current token is not the end of file
    fn not_eof(&self) -> bool {
        self.current < self.tokens.len() && self.tokens[self.current].token_type != TokenType::EOF
    }

    // at returns the current token
    fn at(&self) -> &Token {
        &self.tokens[self.current]
    }

    // eat consumes the current token and moves to the next one
    fn eat(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }

    // expect consumes the current token and checks if it's of the expected type
    fn expect(&mut self, token_type: TokenType) -> Token {
        let token = self.eat();
        if token.token_type != token_type {
            panic!("Unexpected token: {:?}", token);
        }
        token
    }

    // produce_ast generates the Abstract Syntax Tree (AST) from the source code
    pub fn produce_ast(&mut self, source_code: &str) -> Program {
        self.tokens = tokenize(source_code);
        let mut program = Program { body: Vec::new() };

        while self.not_eof() {
            program.body.push(self.parse_stmt());
        }

        program
    }

    // parse_stmt handles complex statement types
    fn parse_stmt(&mut self) -> Box<dyn Stmt> {
        Box::new(ExprStmt { expr: self.parse_expr() })
    }

    // parse_expr handles expressions
    fn parse_expr(&mut self) -> Box<dyn Expr> {
        self.parse_additive_expr()
    }

    // parse_additive_expr handles addition and subtraction operations
    fn parse_additive_expr(&mut self) -> Box<dyn Expr> {
        let mut left = self.parse_multiplicative_expr();

        while self.at().value == "+" || self.at().value == "-" {
            let operator = self.eat().value;
            let right = self.parse_multiplicative_expr();
            let expr = BinaryExpr { left: left.clone_box(), right: right.clone_box(), operator: operator };
            left = Box::new(expr);
        }

        left
    }

    // parse_multiplicative_expr handles multiplication, division, and modulo operations
    fn parse_multiplicative_expr(&mut self) -> Box<dyn Expr> {
        let mut left = self.parse_primary_expr();
    
        while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" {
            let operator = self.eat().value;
            let right = self.parse_primary_expr();
            let expr = BinaryExpr { left: left.clone_box(), right: right, operator: operator };
            left = Box::new(expr);
        }
    
        left
    }

    // parse_primary_expr parses literal values and grouping expressions
    fn parse_primary_expr(&mut self) -> Box<dyn Expr> {
        match self.at().token_type {
            TokenType::Identifier => {
                let symbol = self.eat().value;
                Box::new(Identifier { symbol })
            }
            TokenType::Number => {
                let value = self.eat().value.parse().unwrap();
                Box::new(NumericLiteral { value })
            }
            TokenType::OpenParen => {
                self.eat(); // eat the opening paren
                let value = self.parse_expr();
                self.expect(TokenType::CloseParen);
                value
            }
            _ => panic!("Unexpected token: {:?}", self.at()),
        }
    }
}