use std::io::{self, BufRead};

use monkey_rust::{lexer::Lexer, object, parser::Parser};

fn main() -> anyhow::Result<()> {
    let lines = io::stdin().lock().lines();
    let mut environment = object::Environment::new();
    for line in lines {
        let line = line.expect("Unable to read line from stdin");

        if line.is_empty() {
            println!("Empty line, exiting");
            break;
        }
        let parser = Parser::new(Lexer::new(&line));
        if !parser.errors.is_empty() {
            println!("errors: {:?}", parser.errors);
        }
        if parser.nodes.is_empty() {
            continue;
        }
        let result = environment.eval_program(parser);

        println!("output: {:?}", result);
    }
    Ok(())
}
