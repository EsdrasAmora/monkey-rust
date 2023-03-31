pub(crate) mod ast;

use std::iter::Peekable;

use anyhow::{anyhow, ensure, Error, Result};
use smol_str::SmolStr;

//TODO: how to rexport this?
use crate::lexer::token::Token;
use crate::lexer::Lexer;

use self::ast::{Literal, Statement};

struct Parser {
    nodes: Vec<Statement>,
    errors: Vec<Error>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Vec::with_capacity(32);
        let mut errors = Vec::with_capacity(8);
        let mut tokens = lexer.tokens.into_iter().peekable();

        while let Some(current) = tokens.next() {
            Self::new_helper(current, &mut tokens)
                .map_or_else(|err| errors.push(err), |val| nodes.push(val))
        }

        Parser { nodes, errors }
    }

    //maybe use https://docs.rs/enum-as-inner/0.5.1/enum_as_inner/
    fn new_helper(
        current: Token,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Statement> {
        match current {
            Token::Let => {
                //find a way to peak them consume the iterator;
                let name = tokens
                    .next_if(Token::is_identifier)
                    .and_then(Token::into_identifier)
                    .ok_or(anyhow!(
                        "Expected token to be {:?}, but got {:?} instead",
                        Token::Identifier(SmolStr::default()),
                        tokens.peek(),
                    ))?;

                ensure!(
                    tokens.next_if_eq(&Token::Assign).is_some(),
                    "Expected assign after identifier found: {:?}",
                    tokens.peek()
                );

                while tokens.next().filter(|x| x != &Token::Semicolon).is_some() {}

                Ok(Statement::Let {
                    identifier: name,
                    value: Box::new(Literal::Int(5).into()),
                })
            }
            Token::Return => {
                while tokens.next().filter(|x| x != &Token::Semicolon).is_some() {}
                Ok(Statement::Return(Box::new(Literal::Int(-1).into())))
            }
            _ => {
                let expression = current.parse_expression(tokens, 1)?;
                tokens.next_if_eq(&Token::Semicolon);
                Ok(Statement::Expression(Box::new(expression)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{BinaryExpression, Expression};
    use assert_json_diff::assert_json_eq;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use smol_str::SmolStr;

    #[test]
    fn check_parser_precedence() {
        let input = r#"
        3 + 4 * 5 == 3 * 1 + 4 * 5;"#;
        let right = json!( [{
            "Expression": {
              "Eq": {
                "lhs": {
                  "Add": {
                    "lhs": {
                      "Literal": {
                        "Int": 3
                      }
                    },
                    "rhs": {
                      "Mul": {
                        "lhs": {
                          "Literal": {
                            "Int": 4
                          }
                        },
                        "rhs": {
                          "Literal": {
                            "Int": 5
                          }
                        }
                      }
                    }
                  }
                },
                "rhs": {
                  "Add": {
                    "lhs": {
                      "Mul": {
                        "lhs": {
                          "Literal": {
                            "Int": 3
                          }
                        },
                        "rhs": {
                          "Literal": {
                            "Int": 1
                          }
                        }
                      }
                    },
                    "rhs": {
                      "Mul": {
                        "lhs": {
                          "Literal": {
                            "Int": 4
                          }
                        },
                        "rhs": {
                          "Literal": {
                            "Int": 5
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
        }]);
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert_json_eq!(serde_json::to_value(&program.nodes).unwrap(), right);
    }

    #[test]
    fn parse_infix_expression() {
        let input = "
        5 + 5;
        5 - 5;
        5 * 5;
        5 / 5;
        5 > 5;
        5 < 5;
        5 == 5;
        5 != 5;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);

        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_eq!(
            program.nodes,
            [
                Statement::Expression(
                    Expression::Add(BinaryExpression {
                        lhs: Literal::Int(5).into_exp(),
                        rhs: Literal::Int(5).into_exp()
                    })
                    .boxed()
                ),
                Statement::Expression(
                    Expression::Sub(BinaryExpression {
                        lhs: Literal::Int(5).into_exp(),
                        rhs: Literal::Int(5).into_exp()
                    })
                    .boxed()
                ),
                Statement::Expression(
                    Expression::Mul(BinaryExpression {
                        lhs: Literal::Int(5).into_exp(),
                        rhs: Literal::Int(5).into_exp()
                    })
                    .boxed()
                ),
                Statement::Expression(
                    Expression::Div(BinaryExpression {
                        lhs: Literal::Int(5).into_exp(),
                        rhs: Literal::Int(5).into_exp()
                    })
                    .boxed()
                ),
                Statement::Expression(
                    Expression::Gt(BinaryExpression {
                        lhs: Literal::Int(5).into_exp(),
                        rhs: Literal::Int(5).into_exp()
                    })
                    .boxed()
                ),
                Statement::Expression(
                    Expression::Lt(BinaryExpression {
                        lhs: Literal::Int(5).into_exp(),
                        rhs: Literal::Int(5).into_exp()
                    })
                    .boxed()
                ),
                Statement::Expression(
                    Expression::Eq(BinaryExpression {
                        lhs: Literal::Int(5).into_exp(),
                        rhs: Literal::Int(5).into_exp()
                    })
                    .boxed()
                ),
                Statement::Expression(
                    Expression::NotEq(BinaryExpression {
                        lhs: Literal::Int(5).into_exp(),
                        rhs: Literal::Int(5).into_exp()
                    })
                    .boxed()
                )
            ]
        )
    }

    #[test]
    fn parse_prefix_expression() {
        let input = "
        -1;
        !5
        !!-2;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_eq!(
            program.nodes,
            [
                Statement::Expression(Box::new(Expression::Oposite(Box::new(
                    Literal::Int(1).into()
                )))),
                Statement::Expression(Box::new(Expression::Not(Box::new(Literal::Int(5).into())))),
                Statement::Expression(Box::new(Expression::Not(Box::new(Expression::Not(
                    Box::new(Expression::Oposite(Box::new(Literal::Int(2).into())))
                )))))
            ]
        )
    }

    #[test]
    fn parse_expression_statement_identifier() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_eq!(
            program.nodes,
            [Statement::Expression(Box::new(Expression::Identifier(
                SmolStr::new("foobar")
            )))]
        )
    }

    #[test]
    fn parse_expression_statement_integer_literal() {
        let input = "3;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_eq!(
            program.nodes,
            [Statement::Expression(Box::new(Literal::Int(3).into()))]
        )
    }

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
                    identifier: SmolStr::new("x"),
                    value: Box::new(Literal::Int(5).into())
                },
                Statement::Let {
                    identifier: SmolStr::new("y"),
                    value: Box::new(Literal::Int(5).into())
                },
                Statement::Let {
                    identifier: SmolStr::new("foobar"),
                    value: Box::new(Literal::Int(5).into())
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
                Statement::Return(Box::new(Literal::Int(-1).into())),
                Statement::Return(Box::new(Literal::Int(-1).into())),
                Statement::Return(Box::new(Literal::Int(-1).into()))
            ]
        )
    }
}
