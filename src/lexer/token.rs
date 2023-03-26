#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENTIFIER(String),
    // keywords + literals
    INT(i64),
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    // Operators
    EQ,
    NOTEQ,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
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
