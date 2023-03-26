use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT(String),
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

impl TryFrom<&str> for Token {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let token = match value {
            "=" => Self::ASSIGN,
            "+" => Self::PLUS,
            "," => Self::COMMA,
            ";" => Self::SEMICOLON,
            "(" => Self::LPAREN,
            ")" => Self::RPAREN,
            "{" => Self::LBRACE,
            "}" => Self::RBRACE,
            "fn" => Self::FUNCTION,
            "let" => Self::LET,
            _ => bail!("Unknown token: {}", value),
        };

        Ok(token)
    }
}

impl Token {
    fn as_str(&self) -> Option<&'static str> {
        let str = match self {
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
            _ => return None,
        };

        Some(str)
    }
}
