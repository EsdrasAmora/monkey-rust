use monkey_rust::lexer::visibility_test;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    visibility_test();
    while let Some(line) = lines.next() {
        let last_input = line.expect("Unable to read line from stdin");

        if last_input.len() == 0 {
            println!("Empty line, exiting");
            break;
        }
    }
}
