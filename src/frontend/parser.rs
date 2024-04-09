use std::panic;
use crate::frontend::lexer::*;
use crate::frontend::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        //println!("{:?}", tokens);
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

    fn parse_block(&mut self) -> Block {
        let mut statements = Vec::new();
    
        self.expect(TokenType::OpenBrace);
    
        while self.at().token_type != TokenType::CloseBrace {
            statements.push(self.parse_stmt());
        }
    
        if self.not_eof() {
            self.expect(TokenType::CloseBrace);
        }
    
        Block { statements }
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

        Program { body }
    }
    
    fn parse_stmt(&mut self) -> Stmt {
        let stmt = match self.at().token_type {
            TokenType::If => {
                self.eat();
                let expr = self.parse_if_else_expr();
                Stmt::Expr(expr)
            }
            TokenType::Else => {
                self.eat();
                let expr = self.parse_if_else_expr();
                Stmt::Expr(expr)
            }
            TokenType::While => {
                self.eat();
                let condition = self.parse_expr();
                let body = self.parse_block();
                Stmt::WhileLoop(WhileLoop {
                    condition: Box::new(condition),
                    body,
                })
            }
            TokenType::For => {
                self.eat();
                self.parse_for_loop()
            }
            TokenType::Let => {
                let stmt = self.parse_var_declaration();
                self.expect_semicolon();
                stmt
            }
            TokenType::Const => {
                let stmt = self.parse_var_declaration();
                self.expect_semicolon();
                stmt
            }
            TokenType::Fn => {
                let stmt = self.parse_fn_declaration();
                stmt
            }
            _ => Stmt::Expr(self.parse_expr()),
        };
    
        stmt
    }

    fn expect_semicolon(&mut self) {
        match self.expect(TokenType::SemiColon) {
            Some(_) => (),
            None => panic!("Odotettu ';' lausunnon jälkeen"),
        }
    }
    
    fn parse_fn_declaration(&mut self) -> Stmt {
        self.eat();
        let name = match self.expect(TokenType::Identifier) {
            Some(token) => token.value,
            None => panic!("Odotettu funktion nimi funktio-avainsanan jälkeen"),
        };

        let args = self.parse_args();
        let mut params: Vec<String> = Vec::new();
        for arg in args {
            match arg {
                Expr::Identifier(identifier) => params.push(identifier.symbol),
                _ =>
                    panic!("Funktiomäärityksen sisällä olevien parametrien odotetaan olevan tyyppiä merkkijono"),
            }
        }

        match self.expect(TokenType::OpenBrace) {
            Some(_) => (),
            None => panic!("Odotettu toimintorunko ilmoituksen jälkeen"),
        }
        let mut body: Vec<Stmt> = Vec::new();

        while self.not_eof() && self.at().token_type != TokenType::CloseBrace {
            body.push(self.parse_stmt());
        }

        match self.expect(TokenType::CloseBrace) {
            Some(_) => (),
            None => panic!("Sulkevaa aaltosuljetta odotetaan funktion määrittelyssä"),
        }

        Stmt::FunctionDeclaration(FunctionDeclaration {
            name,
            parameters: params,
            body,
        })
    }

    fn parse_if_else_expr(&mut self) -> Expr {

        let condition = Box::new(self.parse_expr());
    
        let if_branch = self.parse_block();
    
        let else_branch = if self.at().token_type == TokenType::Else {
            self.eat(); 
            Some(self.parse_block())
        } else {
            None
        };
    
        Expr::IfElseExpr(IfElseExpr { condition, if_branch, else_branch })
    }

    fn parse_for_loop(&mut self) -> Stmt {
        self.eat();
        self.expect(TokenType::OpenParen);
        let identifier = match self.expect(TokenType::Identifier) {
            Some(token) => token.value,
            None => panic!("Odotettu tunnisteen nimi avainsanan 'olkoon' jälkeen"),
        };
        self.expect(TokenType::Assign);
        let initialization = Box::new(self.parse_expr());
        self.expect(TokenType::SemiColon);
        let condition = Box::new(self.parse_expr());
        self.expect(TokenType::SemiColon);
        let increment = Box::new(self.parse_expr());
        self.expect(TokenType::CloseParen);
        let body = self.parse_block(); 
        Stmt::ForLoop(ForLoop {
            initializer: Box::new(Stmt::VarDeclaration(VarDeclaration {
                identifier: Identifier { symbol: identifier },
                constant: false,
                value: Some(*initialization),
            })),
            condition: condition,
            increment: increment,
            body: body,
        })
    }

    // Entry point for parsing an expression
    // Calls parse_assignment_expr
    fn parse_expr(&mut self) -> Expr {
        // Skip over any comment tokens
        while self.at().token_type == TokenType::SingleLineComment || self.at().token_type == TokenType::MultiLineComment {
            self.eat();
        }
    
        match self.at().token_type {
            TokenType::If | TokenType::Else => self.parse_if_else_expr(),
            TokenType::OpenBrace => self.parse_object_expr(),
            _ => self.parse_assignment_expr(),
        }
    }

    // Parses variable declarations
    // Calls parse_expr
    fn parse_var_declaration(&mut self) -> Stmt {
        let is_constant = self.eat().token_type == TokenType::Const;
        let identifier = match self.expect(TokenType::Identifier) {
            Some(token) =>
                Identifier {
                    symbol: token.value,
                },
            None => panic!("Odotettu tunnisteen nimi seuraten olkoon | vakio avainsanoja"),
        };
    
        let value = if self.at().token_type == TokenType::Assign {
            self.expect(TokenType::Assign);
            Some(self.parse_expr())
        } else {
            if is_constant {
                panic!("Vakiolausekkeelle on annettava arvo ja arvoa ei ole annettu");
            } else {
                None
            }
        };
    
        Stmt::VarDeclaration(VarDeclaration {
            identifier,
            constant: is_constant,
            value,
        })
    }

    // Parses assignment expressions
    // Calls parse_object_expr
    fn parse_assignment_expr(&mut self) -> Expr {
        let left = self.parse_object_expr();
    
        if self.at().token_type == TokenType::Assign {
            self.eat(); // advance past equals
            let value = self.parse_assignment_expr();
            return Expr::AssignmentExpr(AssignmentExpr {
                value: Box::new(value),
                assignee: Box::new(left),
            });
        }
    
        left
    }

    // Parses array expressions
    // Calls parse_object_expr
    fn parse_array_expr(&mut self) -> Expr {
        if self.at().token_type != TokenType::OpenBracket {
            return self.parse_object_expr();
        }

        self.eat(); // advance past open bracket.
        let mut elements: Vec<Box<Expr>> = Vec::new();

        while self.not_eof() && self.at().token_type != TokenType::CloseBracket {
            let element = self.parse_expr();
            elements.push(Box::new(element));

            if self.at().token_type != TokenType::CloseBracket {
                match self.expect(TokenType::Comma) {
                    Some(_) => (),
                    None => panic!("Odotettu pilkku tai sulkeva hakasulku taulukon elementin jälkeen"),
                };
            }
        }

        match self.expect(TokenType::CloseBracket) {
            Some(_) => (),
            None => panic!("Taulukko literaali puuttuu sulkevasta hakasulusta"),
        }

        Expr::ArrayLiteral(ArrayLiteral { elements })
    }

    // Parses object expressions
    // Calls parse_comparison_expr
    fn parse_object_expr(&mut self) -> Expr {
        if self.at().token_type != TokenType::OpenBrace {
            return self.parse_comparison_expr();
        }

        self.eat(); // advance past open brace.
        let mut properties: Vec<Property> = Vec::new();

        while self.not_eof() && self.at().token_type != TokenType::CloseBrace {
            let key = match self.expect(TokenType::Identifier) {
                Some(token) => token.value,
                None => panic!("Olio literaali odottaa avainta"),
            };

            // Allows shorthand key: pair -> { key, }
            if self.at().token_type == TokenType::Comma {
                self.eat(); // advance past comma
                properties.push(Property { key, value: None });
                continue;
            } else if
                // Allows shorthand key: pair -> { key }
                self.at().token_type == TokenType::CloseBrace
            {
                properties.push(Property { key, value: None });
                continue;
            }

            // { key: val }
            match self.expect(TokenType::Colon) {
                Some(_) => (),
                None => panic!("Oliolauseesta puuttuu kaksoispiste tunnisteesta"),
            }
            let value = self.parse_expr();

            properties.push(Property { value: Some(Box::new(value)), key });
            if self.at().token_type != TokenType::CloseBrace {
                match self.expect(TokenType::Comma) {
                    Some(_) => (),
                    None => panic!("Omaisuuden jälkeen odotettu pilkku tai sulkeva hakasulku"),
                };
            }
        }

        match self.expect(TokenType::CloseBrace) {
            Some(_) => (),
            None => panic!("Olio literaalista puuttuu sulkumerkki"),
        }
        Expr::ObjectLiteral(ObjectLiteral { properties })
    }

    // Parses comparison expressions
    // Calls parse_logical_expr
    fn parse_comparison_expr(&mut self) -> Expr {
        let mut left = self.parse_logical_expr();

        while self.at().value == "==" || self.at().value == "!=" || self.at().value == "<" || self.at().value == ">" || self.at().value == "<=" || self.at().value == ">=" {
            let operator = match self.eat().value.as_str() {
                "==" => BinaryOperator::Equal,
                "!=" => BinaryOperator::NotEqual,
                "<" => BinaryOperator::LessThan,
                ">" => BinaryOperator::GreaterThan,
                "<=" => BinaryOperator::LessThanOrEqual,
                ">=" => BinaryOperator::GreaterThanOrEqual,
                _ => panic!("Odottamaton operaattori"),
            };
            let right = self.parse_logical_expr();
            left = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }

        left
    }

    // Parses logical expressions
    // Calls parse_additive_expr
    fn parse_logical_expr(&mut self) -> Expr {
        let mut left = self.parse_additive_expr();
    
        while self.at().value == "&&" || self.at().value == "||" {
            let operator = match self.eat().value.as_str() {
                "&&" => BinaryOperator::And,
                "||" => BinaryOperator::Or,
                _ => panic!("Odottamaton operaattori"),
            };
            let right = self.parse_additive_expr();
            left = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }
    
        left
    }

    // Parses additive expressions
    // Calls parse_multiplicative_expr
    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_multiplicative_expr();
    
        while self.at().value == "+" || self.at().value == "-" || self.at().value == "+=" || self.at().value == "-=" {
            let operator = match self.eat().value.as_str() {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Subtract,
                "+=" => BinaryOperator::AddEqual,
                "-=" => BinaryOperator::SubtractEqual,
                _ => panic!("Odottamaton operaattori"),
            };
    
            let right = self.parse_multiplicative_expr();
    
            if operator == BinaryOperator::Add || operator == BinaryOperator::Subtract {
                left = Expr::BinaryExpr(BinaryExpr {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator,
                });
            } else {
                let new_value = Expr::BinaryExpr(BinaryExpr {
                    left: Box::new(left.clone()),
                    right: Box::new(right.clone()),
                    operator: if operator == BinaryOperator::AddEqual {
                        BinaryOperator::Add
                    } else {
                        BinaryOperator::Subtract
                    },
                });
    
                left = Expr::AssignmentExpr(AssignmentExpr {
                    assignee: Box::new(left),
                    value: Box::new(new_value),
                });
            }
        }
    
        left
    }

    // Parses multiplicative expressions
    // Calls parse_exponentiation_expr
    fn parse_multiplicative_expr(&mut self) -> Expr {
        let mut left = self.parse_exponentiation_expr();
    
        while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" {
            let operator = match self.eat().value.as_str() {
                "*" => BinaryOperator::Multiply,
                "/" => BinaryOperator::Divide,
                "%" => BinaryOperator::Modulus,
                _ => panic!("Odottamaton operaattori"),
            };
            let right = self.parse_exponentiation_expr();
            left = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }
    
        left
    }

    // Parses exponentiation expressions
    // Calls parse_unary_expr
    fn parse_exponentiation_expr(&mut self) -> Expr {
        let mut left = self.parse_unary_expr();
    
        while self.at().value == "**" {
            self.eat(); // advance past **
            let right = self.parse_unary_expr();
            left = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator: BinaryOperator::Exponent,
            });
        }
    
        left
    }

    // Parses unary expressions
    // Calls parse_call_member_expr
    fn parse_unary_expr(&mut self) -> Expr {
        if self.at().value == "-" || self.at().value == "!" {
            let operator = self.eat().value;
            let operand = self.parse_unary_expr();
            return Expr::UnaryExpr(UnaryExpr {
                operator,
                operand: Box::new(operand),
            });
        }

        self.parse_call_member_expr()
    }

    // Parses call member expressions
    // Calls parse_member_expression
    fn parse_call_member_expr(&mut self) -> Expr {
        let member = self.parse_member_expression();

        if self.at().token_type == TokenType::OpenParen {
            return self.parse_call_expr(member);
        }

        member
    }

    // Parses call expressions
    // Calls parse_arguments_list
    fn parse_call_expr(&mut self, caller: Expr) -> Expr {
        self.expect(TokenType::OpenParen);
        let args = if matches!(self.at().token_type, TokenType::CloseParen) {
            Vec::new()
        } else {
            self.parse_arguments_list()
        };
        self.expect(TokenType::CloseParen);

        let mut call_expr = Expr::CallExpr(CallExpr {
            caller: Box::new(caller),
            args,
        });

        while matches!(self.at().token_type, TokenType::OpenParen) {
            call_expr = self.parse_call_expr(call_expr);
        }

        call_expr
    }

    // Parses arguments
    // Calls parse_arguments_list
    fn parse_args(&mut self) -> Vec<Expr> {
        self.expect(TokenType::OpenParen);
        let args = if matches!(self.at().token_type, TokenType::CloseParen) {
            Vec::new()
        } else {
            self.parse_arguments_list()
        };

        self.expect(TokenType::CloseParen);
        args
    }

    // Parses arguments list
    // Calls parse_assignment_expr
    fn parse_arguments_list(&mut self) -> Vec<Expr> {
        let mut args = vec![self.parse_assignment_expr()];

        while matches!(self.at().token_type, TokenType::Comma) {
            self.eat();
            args.push(self.parse_assignment_expr());
        }

        args
    }

    // Parses member expressions
    // Calls parse_primary_expr
    fn parse_member_expression(&mut self) -> Expr {
        let mut object = self.parse_primary_expr();

        while
            matches!(self.at().token_type, TokenType::Dot) ||
            matches!(self.at().token_type, TokenType::OpenBracket)
        {
            let operator = self.eat();
            let mut property: Box<Expr>;
            let mut computed: bool;

            if operator.token_type == TokenType::Dot {
                computed = false;
                property = Box::new(self.parse_primary_expr());

                if let Expr::Identifier(_) = *property {
                    // property is an identifier
                } else {
                    panic!("Odotettu tunniste pisteoperaattorin jälkeen");
                }
            } else {
                computed = true;
                property = Box::new(self.parse_expr());
                self.expect(TokenType::CloseBracket);
            }

            object = Expr::MemberExpr(MemberExpr {
                object: Box::new(object),
                property,
                computed,
            });
        }

        object
    }

    // Parses primary expressions
    // Calls parse_array_expr
    fn parse_primary_expr(&mut self) -> Expr {
        match self.at().token_type {
            TokenType::Identifier => {
                let symbol = self.eat().value;
                Expr::Identifier(Identifier { symbol })
            }
            TokenType::Integer => {
                let value = self.eat().value.parse().unwrap();
                Expr::NumericLiteral(NumericLiteral { value })
            }
            TokenType::Float => {
                let value = self.eat().value.parse().unwrap();
                Expr::FloatLiteral(FloatLiteral { value })
            }
            TokenType::OpenBracket => { self.parse_array_expr() }
            TokenType::OpenParen => {
                self.eat();
                let expr = self.parse_expr();
                self.expect(TokenType::CloseParen);
                expr
            }
            TokenType::StringLiteral => {
                let value = self.eat().value;
                Expr::StringLiteral(StringLiteral { value })
            }

            _ => panic!("Odottamaton tunnus löytyi jäsentämisen aikana! {:?}", self.at()),
        }
    }
}

// NOTE NEVER DELETE THE ORDER OF PRECEDENCE COMMENTS