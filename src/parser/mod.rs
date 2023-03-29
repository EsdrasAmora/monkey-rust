pub(crate) mod ast;

use std::iter::Peekable;

use anyhow::{anyhow, bail, ensure, Error, Result};

//TODO: how to rexport this?
use crate::lexer::token::Token;
use crate::lexer::Lexer;

use self::ast::{Expression, Literal, Statement};

struct Parser {
    nodes: Vec<Statement>,
    errors: Vec<Error>,
}

#[repr(u8)]
enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Vec::with_capacity(32);
        let mut errors = Vec::with_capacity(8);
        let mut tokens = lexer.tokens.into_iter().peekable();

        while let Some(current) = tokens.next() {
            Self::new_helper(&current, &mut tokens)
                .map_or_else(|err| errors.push(err), |val| nodes.push(val))
        }

        Parser { nodes, errors }
    }

    //maybe use https://docs.rs/enum-as-inner/0.5.1/enum_as_inner/
    fn new_helper(
        current: &Token,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Statement> {
        match current {
            Token::Let => {
                //currently does not autoformat lmao: https://github.com/rust-lang/rustfmt/issues/4914
                //find a way to peak them consume the iterator;
                let name = tokens
                    .next_if(Token::is_identifier)
                    .and_then(Token::into_identifier)
                    .ok_or(anyhow!(
                        "Expected token to be {:?}, but got {:?} instead",
                        Token::Identifier("foo".to_owned()),
                        tokens.peek(),
                    ))?;

                ensure!(
                    tokens.next_if_eq(&Token::Assign).is_some(),
                    "Expected assign after indentifier found: {:?}",
                    tokens.peek()
                );

                while tokens.next().filter(|x| x != &Token::Semicolon).is_some() {}

                Ok(Statement::Let {
                    indentifier: name,
                    value: Box::new(Expression::Literal(Literal::Int(5))),
                })
            }
            Token::Return => {
                while tokens.next().filter(|x| x != &Token::Semicolon).is_some() {}

                Ok(Statement::Return(Box::new(Expression::Literal(
                    Literal::Int(-1),
                ))))
            }
            _ => {
                // Ok(Statement::Expression(Box::new(Expression::Literal(
                //     Literal::Int(-1),
                // ))))
                bail!("Cannot parse an statment starting with {:?}", current)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_let_statement() {
        let input = "
        let x = false;
        let y = 10;
        let foobar = true;";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);

        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);

        assert_eq!(
            program.nodes,
            [
                Statement::Let {
                    indentifier: "x".to_string(),
                    value: Box::new(Expression::Literal(Literal::Int(5)))
                },
                Statement::Let {
                    indentifier: "y".to_string(),
                    value: Box::new(Expression::Literal(Literal::Int(5)))
                },
                Statement::Let {
                    indentifier: "foobar".to_string(),
                    value: Box::new(Expression::Literal(Literal::Int(5)))
                }
            ]
        )
    }

    #[test]
    fn parse_return_statement() {
        let input = "
        return 5;
        return 10;
        return 993322;";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);

        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);

        assert_eq!(
            program.nodes,
            [
                Statement::Return(Box::new(Expression::Literal(Literal::Int(-1)))),
                Statement::Return(Box::new(Expression::Literal(Literal::Int(-1)))),
                Statement::Return(Box::new(Expression::Literal(Literal::Int(-1))))
            ]
        )
    }

    #[test]
    fn parse_with_errors() {
        let input = "
        let x 2;
        let a = 5;
        let = 10;
        let 838383;";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);

        assert_eq!(
            program
                .errors
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            [
                "Expected assign after indentifier found: Some(Int(2))",
                "Cannot parse an statment starting with Int(2)",
                "Cannot parse an statment starting with Semicolon",
                "Expected token to be Identifier(\"foo\"), but got Some(Assign) instead",
                "Cannot parse an statment starting with Assign",
                "Cannot parse an statment starting with Int(10)",
                "Cannot parse an statment starting with Semicolon",
                "Expected token to be Identifier(\"foo\"), but got Some(Int(838383)) instead",
                "Cannot parse an statment starting with Int(838383)",
                "Cannot parse an statment starting with Semicolon",
            ]
        );

        assert_eq!(
            program.nodes,
            [Statement::Let {
                indentifier: "a".to_string(),
                value: Box::new(Expression::Literal(Literal::Int(5)))
            }]
        )
    }
}
