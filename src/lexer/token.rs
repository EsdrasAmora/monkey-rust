use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT,
    INT(i64),
    // Operators
    ASSIGN,
    PLUS,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
}

impl TryFrom<char> for Token {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let token = match value {
            '=' => Token::ASSIGN,
            '+' => Token::PLUS,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            _ => bail!("Unknown char token: {}", value),
        };

        Ok(token)
    }
}

impl TryFrom<&str> for Token {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // if value.len() == 1 {
        //     let token:Token = value.chars().next().expect("Expected exactly one char, got none").try_into()?;
        //     return Ok(token);
        // }
        let token = match value {
            "=" => Token::ASSIGN,
            "+" => Token::PLUS,
            "," => Token::COMMA,
            ";" => Token::SEMICOLON,
            "(" => Token::LPAREN,
            ")" => Token::RPAREN,
            "{" => Token::LBRACE,
            "}" => Token::RBRACE,
            "fn" => Self::FUNCTION,
            "let" => Self::LET,
            _ => bail!("Unknown char token: {}", value),
        };

        Ok(token)
    }
}

impl Token {
    fn as_str(&self) -> Option<&'static str> {
        let str = match self {
            Self::ILLEGAL | Self::EOF | Self::IDENT | Self::INT(_) => return None,
            Self::ASSIGN => "=",
            Self::PLUS => "+",
            Self::COMMA => ",",
            Self::SEMICOLON => ";",
            Self::LPAREN => "(",
            Self::RPAREN => ")",
            Self::LBRACE => "{",
            Self::RBRACE => "}",
            Self::FUNCTION => "fn",
            Self::LET => "let",
        };

        Some(str)
    }
}
