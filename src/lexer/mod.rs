mod token;
use smol_str::SmolStr;
use token::Token;

#[allow(dead_code)]
pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut chars = input.trim().chars().filter(|x| x.is_ascii()).peekable();

        let mut temp = String::with_capacity(10);
        let mut tokens = Vec::with_capacity(32);

        while let Some(mut char) = chars.next() {
            while char.is_whitespace() {
                char = chars.next().expect("Unexpected EOF");
            }

            let char_token = match char {
                '=' => {
                    if chars.peek().and_then(|x| x.eq(&'=').then_some(x)).is_some() {
                        chars.next();
                        Some(Token::EQ)
                    } else {
                        Some(Token::ASSIGN)
                    }
                }
                '!' => {
                    if chars
                        .peek()
                        .is_some_and(|x| x.eq(&'=').then_some(x))
                        .is_some()
                    {
                        chars.next();
                        Some(Token::NOTEQ)
                    } else {
                        Some(Token::BANG)
                    }
                }
                '+' => Some(Token::PLUS),
                ',' => Some(Token::COMMA),
                ';' => Some(Token::SEMICOLON),
                '(' => Some(Token::LPAREN),
                ')' => Some(Token::RPAREN),
                '{' => Some(Token::LBRACE),
                '}' => Some(Token::RBRACE),
                _ => None,
            };

            if let Some(token) = char_token {
                tokens.push(token);
                continue;
            }

            if char.is_ascii_alphabetic() || char == '_' {
                temp.push(char);

                while let Some(next_char) = chars.peek() {
                    if next_char.is_ascii_alphanumeric() || next_char == &'_' {
                        temp.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                let token = match temp.as_str() {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    _ => Token::IDENT(temp.clone()),
                };

                temp.clear();
                tokens.push(token);
                continue;
            }

            if char.is_ascii_digit() {
                temp.push(char);

                while chars
                    .peek()
                    .and_then(|x| x.is_ascii_digit().then_some(x))
                    .is_some()
                {
                    temp.push(chars.next().unwrap());
                }

                tokens.push(Token::INT(temp.clone().parse().unwrap()));
            }
        }

        Lexer { tokens }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_123() {
        let input = r#" 
        let five = 5;
        let ten = 1010;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);"#;

        let lexer = Lexer::new(input);

        assert_eq!(
            lexer.tokens,
            vec![
                Token::LET,
                Token::IDENT("five".to_owned()),
                Token::ASSIGN,
                Token::INT(5),
                Token::SEMICOLON,
                Token::LET,
                Token::IDENT("ten".to_owned()),
                Token::ASSIGN,
                Token::INT(1010),
                Token::SEMICOLON,
                Token::LET,
                Token::IDENT("add".to_owned()),
                Token::ASSIGN,
                Token::FUNCTION,
                Token::LPAREN,
                Token::IDENT("x".to_owned()),
                Token::COMMA,
                Token::IDENT("y".to_owned()),
                Token::RPAREN,
                Token::LBRACE,
                Token::IDENT("x".to_owned()),
                Token::PLUS,
                Token::IDENT("y".to_owned()),
                Token::SEMICOLON,
                Token::RBRACE,
                Token::SEMICOLON,
                Token::LET,
                Token::IDENT("result".to_owned()),
                Token::ASSIGN,
                Token::IDENT("add".to_owned()),
                Token::LPAREN,
                Token::IDENT("five".to_owned()),
                Token::COMMA,
                Token::IDENT("five".to_owned()),
                Token::RPAREN,
                Token::SEMICOLON
            ]
        )
    }
}
