use core::panic;
use crate::frontend::lexer::*;
use crate::frontend::ast::*; 

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

    fn expect(&mut self, token_type: TokenType) -> Option<Token> {
        let token = self.eat();
        if token.token_type == token_type {
            Some(token)
        } else {
            None
        }
    }

    pub fn produce_ast(&mut self) -> Program {
        let mut body = Vec::new();
    
        while self.not_eof() {
            body.push(self.parse_stmt());
        }
    
        Program { kind: NodeType::Program, body }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.at().token_type {
            TokenType::Let => self.parse_var_declaration(),
            TokenType::Const => self.parse_var_declaration(),
            _ => Stmt::Expr(self.parse_expr()),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Expr {
        let left = self.parse_object_expr();
    
        if self.at().token_type == TokenType::Equals {
            self.eat(); // advance past equals
            let value = self.parse_assignment_expr();
            return Expr::AssignmentExpr(AssignmentExpr {
                value: Box::new(value),
                assignee: Box::new(left),
                kind: NodeType::AssignmentExpr,
            });
        }
    
        left
    }

    fn parse_object_expr(&mut self) -> Expr {
        if self.at().token_type != TokenType::OpenBrace {
            return self.parse_additive_expr();
        }
    
        self.eat();
        let mut properties: Vec<Property> = Vec::new();
    
        while self.not_eof() && self.at().token_type != TokenType::CloseBrace {
            let key = self.expect(TokenType::Identifier).unwrap().value;

            // Allows shorthand property assignment { key, }
            if self.at().token_type == TokenType::Comma {
                self.eat();
                properties.push(Property {
                    key,
                    kind: NodeType::Property,
                    value: None,
                });
                continue;
            } 
            // Allows shorthand property assignment { key }
            if self.at().token_type == TokenType::CloseBrace {
                self.eat();
                properties.push(Property {
                    key,
                    kind: NodeType::Property,
                    value: None,
                });
                continue;
            }
            // { key: val }
            self.expect(TokenType::Colon);
            let value = self.parse_expr();

            properties.push(Property {
                key,
                kind: NodeType::Property,
                value: Some(Box::new(value)),
            });
            if self.at().token_type != TokenType::CloseBrace {
                self.expect(TokenType::Comma);
            }
        }
    
        self.expect(TokenType::CloseBrace);
    
        Expr::ObjectLiteral(ObjectLiteral {
            kind: NodeType::ObjectLiteral,
            properties,
        })
    }

    fn parse_var_declaration(&mut self) -> Stmt {
        let is_constant = self.eat().token_type == TokenType::Const;
        let identifier = match self.expect(TokenType::Identifier) {
            Some(token) => Identifier { 
                kind: NodeType::Identifier, 
                symbol: token.value 
            },
            None => panic!("Expected identifier name following let | const keywords."),
        };
    
        if self.at().token_type == TokenType::SemiColon {
            if is_constant {
                panic!("Must assign value to constant expression. No value provided.");
            } else {
                self.eat();
                return Stmt::VarDeclaration(VarDeclaration {
                    kind: NodeType::VarDeclaration,
                    identifier,
                    constant: false,
                    value: None,
                });
            }
        }
    
        self.expect(TokenType::Equals);
        let value = Some(self.parse_expr());
    
        if self.at().token_type == TokenType::SemiColon {
            self.eat();
        } else {
            panic!("Expected ';' after variable declaration");
        }
    
        Stmt::VarDeclaration(VarDeclaration {
            kind: NodeType::VarDeclaration,
            identifier: identifier.clone(),
            constant: is_constant,
            value,
        })
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
            _ => panic!("Unexpected token found during parsing! {:?}", self.at()),
        }
    }
}