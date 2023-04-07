use std::io::{self, BufRead};

use monkey_rust::{lexer::Lexer, parser::Parser};

fn main() -> anyhow::Result<()> {
    let lines = io::stdin().lock().lines();
    for line in lines {
        let line = line.expect("Unable to read line from stdin");

        if line.is_empty() {
            println!("Empty line, exiting");
            break;
        }
        let program = Parser::new(Lexer::new(&line));
        if !program.errors.is_empty() {
            println!("errors: {:?}", program.errors);
        }
        if !program.nodes.is_empty() {
            println!("nodes: {:?}", program.nodes);
        }
    }
    Ok(())
}
