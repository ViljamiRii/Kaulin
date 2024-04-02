use crate::frontend::lexer::*;
use crate::frontend::ast::{Expr, BinaryExpr, Identifier, NumericLiteral, NodeType, Program, Stmt}; // Import the types from ast.rs

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    fn not_eof(&self) -> bool {
        !matches!(self.tokens.first(), Some(Token { token_type: TokenType::EOF, .. }))
    }

    fn at(&self) -> &Token {
        &self.tokens[0]
    }

    fn eat(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn expect(&mut self, token_type: TokenType) -> Token {
        let token = self.eat();
        assert_eq!(token.token_type, token_type);
        token
    }

    pub fn produce_ast(&mut self) -> Program {
        let mut body = Vec::new();
    
        while self.not_eof() {
            body.push(Stmt::Expr(self.parse_stmt()));
        }
    
        Program { kind: NodeType::Program, body }
    }

    fn parse_stmt(&mut self) -> Expr {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_multiplicative_expr();
    
        while self.at().value == "+" || self.at().value == "-" {
            let operator = self.eat().value;
            let right = self.parse_multiplicative_expr();
            left = Expr::BinaryExpr(BinaryExpr {
                kind: NodeType::BinaryExpr,
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }
    
        left
    }

    fn parse_multiplicative_expr(&mut self) -> Expr {
        let mut left = self.parse_primary_expr();
    
        while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" {
            let operator = self.eat().value;
            let right = self.parse_primary_expr();
            left = Expr::BinaryExpr(BinaryExpr {
                kind: NodeType::BinaryExpr,
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }
    
        left
    }

    fn parse_primary_expr(&mut self) -> Expr {
        match self.at().token_type {
            TokenType::Identifier => {
                let symbol = self.eat().value;
                Expr::Identifier(Identifier { kind: NodeType::Identifier, symbol })
            }
            TokenType::Number => {
                let value = self.eat().value.parse().unwrap();
                Expr::NumericLiteral(NumericLiteral { kind: NodeType::NumericLiteral, value })
            }
            TokenType::OpenParen => {
                self.eat();
                let expr = self.parse_expr();
                self.expect(TokenType::CloseParen);
                expr
            }
            _ => panic!("Unexpected token found during parsing!"),
        }
    }
}