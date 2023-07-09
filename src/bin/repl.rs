use monkey_rust::lexer::Lexer;
use monkey_rust::object::Environment;
use monkey_rust::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut env = Environment::new();
    // let mut buffer = vec![];

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let lexer = Lexer::new(&line);
                let parser = Parser::new(lexer);
                let result = env.eval_program(parser);
                println!("{:?}", result);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
