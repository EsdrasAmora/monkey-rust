use crate::token::{Identifier, Token};
use smol_str::SmolStr;
use std::iter::{self, Peekable};

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut chars = input.trim().chars().filter(|x| x.is_ascii()).peekable();
        let mut tokens = Vec::with_capacity(32);

        while let Some(char) = chars.next() {
            if char.is_whitespace() {
                continue;
            }
            let token = Lexer::new_helper(char, &mut chars);
            tokens.push(token);
        }

        Lexer { tokens }
    }

    fn new_helper(char: char, chars: &mut Peekable<impl Iterator<Item = char>>) -> Token {
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
            '<' => chars.next_if_eq(&'=').map_or(Token::Lt, |_| Token::Lte),
            '>' => chars.next_if_eq(&'=').map_or(Token::Gt, |_| Token::Gte),
            '=' => chars.next_if_eq(&'=').map_or(Token::Assign, |_| Token::Eq),
            '!' => chars.next_if_eq(&'=').map_or(Token::Bang, |_| Token::NotEq),
            '"' => {
                let string: SmolStr = iter::from_fn(|| chars.next_if(|x| *x != '"')).collect();
                chars.next();
                Token::String(string)
            }
            _ if char.is_ascii_alphabetic() || char == '_' => {
                let keyword: SmolStr = iter::once(char)
                    .chain(iter::from_fn(|| {
                        chars.next_if(|x| x.is_ascii_alphanumeric() || *x == '_')
                    }))
                    .collect();

                match keyword.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "nil" => Token::Nil,
                    _ => Token::Identifier(Identifier::new(keyword)),
                }
            }
            _ if char.is_ascii_digit() => iter::once(char)
                .chain(iter::from_fn(|| chars.next_if(|x| x.is_ascii_digit())))
                .collect::<SmolStr>()
                .parse()
                .map_or(Token::Illegal, Token::Int),
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
        10 != 9;
        "foobar"
        "foo bar"
        "#;

        let lexer = Lexer::new(input);

        assert_eq!(
            lexer.tokens,
            vec![
                Token::Let,
                Token::Identifier(SmolStr::new("five").into()),
                Token::Assign,
                Token::Int(5),
                Token::Semicolon,
                Token::Let,
                Token::Identifier(SmolStr::new("ten").into()),
                Token::Assign,
                Token::Int(10),
                Token::Semicolon,
                Token::Let,
                Token::Identifier(SmolStr::new("add").into()),
                Token::Assign,
                Token::Function,
                Token::LParen,
                Token::Identifier(SmolStr::new("x").into()),
                Token::Comma,
                Token::Identifier(SmolStr::new("y").into()),
                Token::RParen,
                Token::LBrace,
                Token::Identifier(SmolStr::new("x").into()),
                Token::Plus,
                Token::Identifier(SmolStr::new("y").into()),
                Token::Semicolon,
                Token::RBrace,
                Token::Semicolon,
                Token::Let,
                Token::Identifier(SmolStr::new("result").into()),
                Token::Assign,
                Token::Identifier(SmolStr::new("add").into()),
                Token::LParen,
                Token::Identifier(SmolStr::new("five").into()),
                Token::Comma,
                Token::Identifier(SmolStr::new("ten").into()),
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
                Token::Semicolon,
                Token::String(SmolStr::new_inline("foobar")),
                Token::String(SmolStr::new_inline("foo bar")),
            ]
        )
    }
}
