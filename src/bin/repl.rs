use monkey_rust::eval::Program;
use monkey_rust::lexer::Lexer;
use monkey_rust::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut program = Program::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                if line == "dbg!" {
                    println!("{:?}", program.env);
                    continue;
                }
                let lexer = Lexer::new(&line);
                let parser = Parser::new(lexer);
                let result = program.eval(parser);
                println!("{:?}", result)
                // serde_json::to_string_pretty(&obj).unwrap()
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
