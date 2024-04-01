mod frontend;

use frontend::parser::Parser;
use std::io::{self, Write};
use std::process;

fn main() {
    repl();
}

fn repl() {
    println!("\nRepl v0.1");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() || input.trim() == "exit" {
            process::exit(1);
        }

        // Create a new Parser for each input
        let mut parser = Parser::new();
        let program = parser.produce_ast(&input);
        println!("{:?}", program);
    }
}