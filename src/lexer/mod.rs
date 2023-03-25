mod token;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Lexer {
    input: String,
    position: u32,      // current position in input (points to current char)
    read_position: u32, // current reading position in input (after current char)
    char: char,         // current char under examination
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer::default()
    }
}

pub fn visibility_test() {
    println!("Hello world, {:?}", token::Token::LET);
}
