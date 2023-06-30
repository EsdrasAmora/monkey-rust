use crate::ast::{BinaryExpression, BlockStatement, Expression, IfExpression, Literal, Statement};
use crate::object::{Environment, Object, NIL};
use crate::parser::Parser;
use anyhow::Result;

impl Environment {
    pub fn eval_program(&mut self, parser: Parser) -> Result<Object> {
        let mut result = Object::Nil;
        for node in parser.nodes {
            result = node.eval_statement()?;
            if let Object::Return(inner) = result {
                return Ok(*inner);
            }
        }
        Ok(result)
    }
}

impl Statement {
    fn eval_statement(self: Statement) -> Result<Object> {
        match self {
            Statement::Let { identifier, value } => todo!(),
            Statement::Return(exp) => Ok(Object::Return(Box::new(exp.eval()?))),
            Statement::Expression(exp) => Ok(exp.eval()?),
        }
    }
}

impl BlockStatement {
    fn eval_block(self) -> Result<Object> {
        let mut result = Object::Nil;
        for node in self.0 {
            result = node.eval_statement()?;
            if let Object::Return(_) = result {
                break;
            }
        }
        Ok(result)
    }
}

impl Expression {
    fn eval(self) -> Result<Object> {
        Ok(match self {
            Expression::Literal(literal) => match literal {
                Literal::Int(integer) => Object::Int(integer),
                Literal::True => Object::Bool(true),
                Literal::False => Object::Bool(false),
                Literal::String(string) => Object::String(string),
                Literal::Nil => Object::Nil,
            },
            Expression::Identifier(ident) => todo!(),
            Expression::Oposite(exp) => exp.0.eval()?.oposite()?,
            Expression::Not(exp) => exp.0.eval()?.not()?,
            Expression::Eq(exp) => exp.lhs.eval()?.eq(exp.rhs.eval()?).into(),
            Expression::NotEq(exp) => exp.lhs.eval()?.not_eq(exp.rhs.eval()?).into(),
            Expression::Lt(exp) => exp.lhs.eval()?.lt(exp.rhs.eval()?).into(),
            Expression::Lte(exp) => exp.lhs.eval()?.lte(exp.rhs.eval()?).into(),
            Expression::Gt(exp) => exp.lhs.eval()?.gt(exp.rhs.eval()?).into(),
            Expression::Gte(exp) => exp.lhs.eval()?.gte(exp.rhs.eval()?).into(),
            Expression::Add(exp) => exp.lhs.eval()?.add(exp.rhs.eval()?)?,
            Expression::Sub(exp) => exp.lhs.eval()?.sub(exp.rhs.eval()?)?,
            Expression::Mul(exp) => exp.lhs.eval()?.mul(exp.rhs.eval()?)?,
            Expression::Div(exp) => exp.lhs.eval()?.div(exp.rhs.eval()?)?,
            Expression::If(exp) => exp.eval()?,
            Expression::Function(_) => todo!(),
            Expression::Call(_) => todo!(),
        })
    }
}

impl BinaryExpression {
    fn eval(self) -> Result<(Object, Object)> {
        Ok((self.lhs.eval()?, self.rhs.eval()?))
    }
}

impl IfExpression {
    //, environment: &mut Environment
    fn eval(self) -> Result<Object> {
        let condition = self.condition.eval()?.into_bool()?;

        Ok(if condition {
            self.consequence.eval_block()?
        } else if let Some(alternative) = self.alternative {
            alternative.eval_block()?
        } else {
            NIL
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{lexer::Lexer, parser::Parser};
    use insta::assert_yaml_snapshot;

    fn parse_program(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let mut environment = Environment::new();

        environment.eval_program(parser).unwrap()
    }

    fn parse_test_input(input: &[&str]) -> Vec<Object> {
        input.iter().map(|x| parse_program(x)).collect()
    }

    #[test]
    fn eval_numeric_expression() {
        let input = [
            "5;",
            "-10;",
            "3 * (3 + 3) + 10;",
            "(5 + 10 * 2 + 15 / 3) * 2 + -10;",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_boolean_expression() {
        let input = [
            "true;",
            "false;",
            "1 >= 2;",
            "1 >= 1;",
            "1 <= 2;",
            "1 <= 1;",
            "1 > 2;",
            "1 > 1;",
            "1 < 2;",
            "1 < 1;",
            "true == true;",
            "false != false;",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_not_expression() {
        let input = [
            "!5;", "!!5;", "!0;", "!!0;", "!true;", "!!true;", "!false;", "!!false;",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_if_expression() {
        let input = [
            "if (true) { 10; };",
            "if (false) { 10; };",
            "if (1) { 10; };",
            "if (1 < 2) { 10; };",
            "if (1 > 2) { 10; };",
            "if (1 > 2) { 10; } else { 20; };",
            "if (1 < 2) { 10; } else { 20; };",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_return_statement() {
        let input = [
            "return 10;",
            "return 10; 9;",
            "return 2 * 5; 9;",
            "9; return 2 * 5; 9;",
            "if (true) { return 10; }; return 9;",
            "if (true) { if (true) { return 10; }; return 1;};",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_type_errors() {
        let input = [
            "5 + true;",
            "5 + true; 5;",
            "-true;",
            "true + false;",
            "true + false + true + false;",
            "5; true + false; 5",
            "if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                };
                return 1;
            };",
        ];

        let result: Vec<String> = input
            .iter()
            .flat_map(|x| {
                let lexer = Lexer::new(x);
                let parser = Parser::new(lexer);
                let mut environment = Environment::new();
                let val = environment.eval_program(parser);
                match val {
                    Ok(val) => None,
                    Err(err) => Some(err.to_string()),
                }
            })
            .collect();

        assert_yaml_snapshot!(result);
    }
}
