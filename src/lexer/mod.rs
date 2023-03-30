pub(crate) mod token;
use std::iter::Peekable;
use token::Token;

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut chars = input.trim().chars().filter(|x| x.is_ascii()).peekable();

        let mut temp = String::with_capacity(10);
        let mut tokens = Vec::with_capacity(32);

        while let Some(mut char) = chars.next() {
            while char.is_whitespace() {
                char = chars.next().expect("Unexpected EOF");
            }
            let token = Lexer::new_helper(char, &mut temp, &mut chars);
            tokens.push(token);
        }

        Lexer { tokens }
    }

    fn new_helper(
        char: char,
        temp: &mut String,
        chars: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Token {
        match char {
            '+' => Token::Plus,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '=' => {
                if chars.peek().filter(|x| x == &&'=').is_some() {
                    chars.next();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if chars.peek().filter(|x| x == &&'=').is_some() {
                    chars.next();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            _ if char.is_ascii_alphabetic() || char == '_' => {
                temp.push(char);

                while let Some(next_char) = chars
                    .peek()
                    .filter(|x| x.is_ascii_alphanumeric() || **x == '_')
                {
                    temp.push(*next_char);
                    chars.next();
                }

                let token = match temp.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Identifier(temp.clone()),
                };

                temp.clear();
                token
            }
            _ if char.is_ascii_digit() => {
                temp.push(char);

                while chars
                    .peek()
                    .filter(|x| x.is_ascii_digit())
                    .map(|x| temp.push(*x))
                    .is_some()
                {
                    chars.next();
                }

                let clone = std::mem::take(temp);
                clone.parse().map_or(Token::Illegal, Token::Int)
            }
            _ => Token::Illegal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_sucessfully() {
        let input = r#" 
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;"#;

        let lexer = Lexer::new(input);

        assert_eq!(
            lexer.tokens,
            vec![
                Token::Let,
                Token::Identifier("five".to_owned()),
                Token::Assign,
                Token::Int(5),
                Token::Semicolon,
                Token::Let,
                Token::Identifier("ten".to_owned()),
                Token::Assign,
                Token::Int(10),
                Token::Semicolon,
                Token::Let,
                Token::Identifier("add".to_owned()),
                Token::Assign,
                Token::Function,
                Token::LParen,
                Token::Identifier("x".to_owned()),
                Token::Comma,
                Token::Identifier("y".to_owned()),
                Token::RParen,
                Token::LBrace,
                Token::Identifier("x".to_owned()),
                Token::Plus,
                Token::Identifier("y".to_owned()),
                Token::Semicolon,
                Token::RBrace,
                Token::Semicolon,
                Token::Let,
                Token::Identifier("result".to_owned()),
                Token::Assign,
                Token::Identifier("add".to_owned()),
                Token::LParen,
                Token::Identifier("five".to_owned()),
                Token::Comma,
                Token::Identifier("ten".to_owned()),
                Token::RParen,
                Token::Semicolon,
                Token::Bang,
                Token::Minus,
                Token::Slash,
                Token::Asterisk,
                Token::Int(5),
                Token::Semicolon,
                Token::Int(5),
                Token::Lt,
                Token::Int(10),
                Token::Gt,
                Token::Int(5),
                Token::Semicolon,
                Token::If,
                Token::LParen,
                Token::Int(5),
                Token::Lt,
                Token::Int(10),
                Token::RParen,
                Token::LBrace,
                Token::Return,
                Token::True,
                Token::Semicolon,
                Token::RBrace,
                Token::Else,
                Token::LBrace,
                Token::Return,
                Token::False,
                Token::Semicolon,
                Token::RBrace,
                Token::Int(10),
                Token::Eq,
                Token::Int(10),
                Token::Semicolon,
                Token::Int(10),
                Token::NotEq,
                Token::Int(9),
                Token::Semicolon
            ]
        )
    }
}
