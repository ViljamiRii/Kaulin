mod runtime;
mod frontend;

use crate::frontend::ast::Stmt;
use crate::frontend::lexer::tokenize;
use crate::frontend::parser::Parser;
use crate::runtime::environment::Environment;
use crate::runtime::interpreter::evaluate;
use crate::runtime::values::{MK_BOOL, MK_NULL, MK_NUMBER};
use std::io::{self, Write};

fn repl() {
    let mut env = Environment::new(None);

    // Create Default Global Environment
    env.declare_var("x".to_string(), MK_NUMBER(100.0));
    env.declare_var("true".to_string(), MK_BOOL(true));
    env.declare_var("false".to_string(), MK_BOOL(false));
    env.declare_var("null".to_string(), MK_NULL());

    // INITIALIZE REPL
    println!("\nRepl v0.1");

    // Continue Repl Until User Stops Or Types `exit`
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Check for no user input or exit keyword.
        if input.trim().is_empty() || input.contains("exit") {
            break;
        }

        // Produce AST From source-code
        let tokens = tokenize(&input);
        let program = Parser::new(tokens).produce_ast();
        let stmt = Stmt::Program(program); // Change this line

        let result = evaluate(stmt, &mut env);
        println!("{:?}", result);
    }
}

fn main() {
    repl();
}