#[derive(Debug, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT,
    INT,
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
