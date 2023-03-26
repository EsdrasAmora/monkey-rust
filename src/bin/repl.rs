use std::io::{self, BufRead};

use monkey_rust::lexer::Lexer;

fn main() {
    let lines = io::stdin().lock().lines();
    for line in lines {
        let line = line.expect("Unable to read line from stdin");

        if line.is_empty() {
            println!("Empty line, exiting");
            break;
        }

        println!("Tokens: {:?}", Lexer::new(&line));
    }
}
