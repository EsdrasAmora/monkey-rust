mod token;

use token::Token;

#[allow(dead_code)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut chars = input
            .char_indices()
            .filter(|(_, x)| x.is_ascii() && !x.is_ascii_whitespace())
            .peekable();

        let temp = String::with_capacity(4);

        while let Some((index, char)) = chars.next() {
            if let Ok(token) = TryInto::<Token>::try_into(char) {
                println!("Found token: {:?}", token);
            }
        }

        Lexer { tokens: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_123() {
        let input = r#" 
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);"#;

        let lexer = Lexer::new(input);

        assert_eq!(lexer.tokens, vec![])
    }
}
