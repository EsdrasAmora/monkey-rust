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
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            ':' => Token::Colon,
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
    use insta::assert_yaml_snapshot;

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
        !-/*5
        5 < 10 > 5

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10
        10 != 9
        "foobar"
        "foo bar"
        [1, 2]
        {"foo": "bar"}
        "#;

        let result = Lexer::new(input);
        assert_yaml_snapshot!(result.tokens);
    }
}
