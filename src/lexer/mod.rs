mod token;
use token::Token;

#[allow(dead_code)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut chars = input
            .trim()
            .char_indices()
            .filter(|(_, x)| x.is_ascii())
            .peekable();

        // println!("Chars: {:?}", chars.collect::<Vec<_>>());

        let mut temp = String::with_capacity(10);
        let mut tokens = Vec::with_capacity(20);

        'outer: while let Some((index, mut char)) = chars.next() {
            while char.is_whitespace() {
                //as it is already trimmed, we could just unwrap here
                if let Some((_, next_char)) = chars.next() {
                    char = next_char;
                } else {
                    break 'outer;
                }
            }

            let char_token = match char {
                '=' => Some(Token::ASSIGN),
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

                while let Some((_, next_char)) = chars.peek() {
                    if next_char.is_ascii_alphanumeric() || next_char == &'_' {
                        temp.push(chars.next().unwrap().1);
                    } else {
                        break;
                    }
                }

                let token = match temp.as_str() {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    _ => {
                        //TODO: just clone it
                        let indentifier = std::mem::replace(&mut temp, String::new());
                        println!("Identifier: {indentifier}, emptyString: {temp}");
                        Token::IDENT(indentifier)
                    }
                };

                temp.clear();
                tokens.push(token);
                continue;
            }

            if char.is_ascii_digit() {
                temp.push(char);

                while let Some((_, next_char)) = chars.peek() {
                    //TODO: allow '_' in numbers
                    if next_char.is_ascii_digit() {
                        temp.push(chars.next().unwrap().1);
                    } else {
                        break;
                    }
                }

                let number = std::mem::replace(&mut temp, String::new());
                tokens.push(Token::INT(number.parse().unwrap()));
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
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);"#;

        let lexer = Lexer::new(input);

        assert_eq!(lexer.tokens, vec![])
    }
}
