#![allow(warnings)]

mod runtime;
mod frontend;

use crate::frontend::ast::*;
use crate::frontend::lexer::*;
use crate::frontend::parser::*;
use crate::runtime::environment::*;
use crate::runtime::interpreter::*;
use crate::runtime::values::*;
use std::io::{self, Write};
use std::fs;

fn run(filename: &str) {
    let global_env = create_global_env();
    let mut env = Environment::new(Some(Box::new(global_env)));

    let input = fs::read_to_string(filename).expect("Could not read file");
    let tokens = tokenize(&input);
    let program = Parser::new(tokens).produce_ast();
    let stmt = Stmt::Program(program);

    let result = evaluate(stmt, &mut env);
    println!("{:?}", result);
}

fn repl() {
    let global_env = create_global_env();
    let mut env = Environment::new(Some(Box::new(global_env)));


    // INITIALIZE REPL
    println!("\nViljami Repl 💀 v0.1");

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
        let stmt = Stmt::Program(program);

        let result = evaluate(stmt, &mut env);
        println!("{:?}", result);
    }
}

fn main() {
    //repl();
    run("./test.txt");
}