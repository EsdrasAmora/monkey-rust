mod token;
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
                '+' => Some(Token::PLUS),
                ',' => Some(Token::COMMA),
                ';' => Some(Token::SEMICOLON),
                '(' => Some(Token::LPAREN),
                ')' => Some(Token::RPAREN),
                '{' => Some(Token::LBRACE),
                '}' => Some(Token::RBRACE),
                '-' => Some(Token::MINUS),
                '*' => Some(Token::ASTERISK),
                '/' => Some(Token::SLASH),
                '<' => Some(Token::LT),
                '>' => Some(Token::GT),
                '=' => {
                    if chars.peek().and_then(|x| x.eq(&'=').then_some(x)).is_some() {
                        chars.next();
                        Some(Token::EQ)
                    } else {
                        Some(Token::ASSIGN)
                    }
                }
                '!' => {
                    //TODO: use `is_some_and` when it's stable: https://github.com/rust-lang/rust/issues/93050#issuecomment-1019312470
                    if chars.peek().and_then(|x| x.eq(&'=').then_some(x)).is_some() {
                        chars.next();
                        Some(Token::NOTEQ)
                    } else {
                        Some(Token::BANG)
                    }
                }
                _ if char.is_ascii_alphabetic() || char == '_' => {
                    temp.push(char);

                    while chars
                        .peek()
                        .and_then(|x| {
                            (x.is_ascii_alphanumeric() || *x == '_').then(|| temp.push(*x))
                        })
                        .is_some()
                    {
                        chars.next();
                    }

                    let token = match temp.as_str() {
                        "fn" => Token::FUNCTION,
                        "let" => Token::LET,
                        "true" => Token::TRUE,
                        "false" => Token::FALSE,
                        "if" => Token::IF,
                        "else" => Token::ELSE,
                        "return" => Token::RETURN,
                        _ => Token::IDENTIFIER(temp.clone()),
                    };

                    temp.clear();
                    Some(token)
                }
                _ if char.is_ascii_digit() => {
                    temp.push(char);

                    while chars
                        .peek()
                        .and_then(|x| x.is_ascii_digit().then(|| temp.push(*x)))
                        .is_some()
                    {
                        chars.next();
                    }

                    let clone = std::mem::replace(&mut temp, String::new());
                    clone.parse().ok().and_then(|x| Some(Token::INT(x)))
                }
                _ => None,
            };

            char_token
                .and_then(|f| Some(tokens.push(f)))
                .or_else(|| Some(tokens.push(Token::ILLEGAL)));
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
                Token::LET,
                Token::IDENTIFIER("five".to_owned()),
                Token::ASSIGN,
                Token::INT(5),
                Token::SEMICOLON,
                Token::LET,
                Token::IDENTIFIER("ten".to_owned()),
                Token::ASSIGN,
                Token::INT(10),
                Token::SEMICOLON,
                Token::LET,
                Token::IDENTIFIER("add".to_owned()),
                Token::ASSIGN,
                Token::FUNCTION,
                Token::LPAREN,
                Token::IDENTIFIER("x".to_owned()),
                Token::COMMA,
                Token::IDENTIFIER("y".to_owned()),
                Token::RPAREN,
                Token::LBRACE,
                Token::IDENTIFIER("x".to_owned()),
                Token::PLUS,
                Token::IDENTIFIER("y".to_owned()),
                Token::SEMICOLON,
                Token::RBRACE,
                Token::SEMICOLON,
                Token::LET,
                Token::IDENTIFIER("result".to_owned()),
                Token::ASSIGN,
                Token::IDENTIFIER("add".to_owned()),
                Token::LPAREN,
                Token::IDENTIFIER("five".to_owned()),
                Token::COMMA,
                Token::IDENTIFIER("ten".to_owned()),
                Token::RPAREN,
                Token::SEMICOLON,
                Token::BANG,
                Token::MINUS,
                Token::SLASH,
                Token::ASTERISK,
                Token::INT(5),
                Token::SEMICOLON,
                Token::INT(5),
                Token::LT,
                Token::INT(10),
                Token::GT,
                Token::INT(5),
                Token::SEMICOLON,
                Token::IF,
                Token::LPAREN,
                Token::INT(5),
                Token::LT,
                Token::INT(10),
                Token::RPAREN,
                Token::LBRACE,
                Token::RETURN,
                Token::TRUE,
                Token::SEMICOLON,
                Token::RBRACE,
                Token::ELSE,
                Token::LBRACE,
                Token::RETURN,
                Token::FALSE,
                Token::SEMICOLON,
                Token::RBRACE,
                Token::INT(10),
                Token::EQ,
                Token::INT(10),
                Token::SEMICOLON,
                Token::INT(10),
                Token::NOTEQ,
                Token::INT(9),
                Token::SEMICOLON
            ]
        )
    }
}
