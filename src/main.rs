#![allow(warnings)]

mod runtime;
mod frontend;

use crate::frontend::ast::*;
use crate::frontend::lexer::*;
use crate::frontend::parser::*;
use crate::runtime::environment::*;
use crate::runtime::interpreter::*;
use crate::runtime::values::*;
use std::io::{ self, Write };
use std::fs;
use std::env;
use std::fmt;

impl fmt::Display for RuntimeVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeVal::Null => write!(f, "null"),
            RuntimeVal::Bool(b) => write!(f, "{}", b),
            RuntimeVal::Integer(i) => write!(f, "{}", i), 
            RuntimeVal::Number(n) => write!(f, "{}", n),
            RuntimeVal::String(s) => write!(f, "{}", s),
            RuntimeVal::Object(obj) => {
                let properties: Vec<String> = obj.iter()
                    .map(|(key, value)| format!("{}: {}", key, value))
                    .collect();
                write!(f, "{{{}}}", properties.join(", "))
            },
            RuntimeVal::Array(arr) => {
                let elements: Vec<String> = arr.iter()
                    .map(|value| format!("{}", value))
                    .collect();
                write!(f, "[{}]", elements.join(", "))
            },
            RuntimeVal::NativeFunction(_) => write!(f, "NativeFunction"),
            RuntimeVal::Function(func) => write!(f, "Function({})", func.parameters.join(", ")),
        }
    }
}

fn repl() {
    let global_env = create_global_env();
    let mut env = Environment::new(Some(Box::new(global_env)));

    // INITIALIZE REPL
    println!("\nKaulin Repl v0.1!\nKirjoita 'exit' tai ' ' poistuaksesi repl:stä.");

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

        let result = evaluate(&stmt, &mut env);
        println!("{:?}", result);

    }
}

fn run(filename: &str) {
    if !filename.ends_with(".ka") {
        println!("Virhe: Virheellinen tiedostomuoto. Anna .ka-tiedosto.");
        return;
    }

    let global_env = create_global_env();
    let mut env = Environment::new(Some(Box::new(global_env)));

    let input = fs::read_to_string(filename).expect("Tiedostoa ei voitu lukea");
    let tokens = tokenize(&input);
    let program = Parser::new(tokens).produce_ast();
    let stmt = Stmt::Program(program);

    let _ = evaluate(&stmt, &mut env);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let filename = &args[1];
            run(filename);
        },
        _ => {
            repl();
        }
    }
}