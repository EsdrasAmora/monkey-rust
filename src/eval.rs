use crate::ast::{
    BlockStatement, Expression, FunctionExpression, IfExpression, Literal, Statement,
};
use crate::object::{Environment, Function, Object, NIL};
use crate::parser::Parser;
use anyhow::{anyhow, Result};

impl Environment {
    pub fn eval_program(&mut self, parser: Parser) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in parser.nodes {
            result = statement.eval(self)?;
            if let Object::Return(inner) = result {
                return Ok(*inner);
            }
        }
        Ok(result)
    }
}

impl Statement {
    fn eval(self: Statement, environment: &mut Environment) -> Result<Object> {
        match self {
            Statement::Let { identifier, value } => {
                let val = value.eval(environment)?;
                if let Some(ident) = environment.store.insert(identifier.inner(), val) {
                    return Err(anyhow!("Identifier {ident:?} already defined"));
                }
                Ok(NIL)
            }
            Statement::Return(exp) => Ok(Object::Return(Box::new(exp.eval(environment)?))),
            Statement::Expression(exp) => Ok(exp.eval(environment)?),
        }
    }
}

impl BlockStatement {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in self.0 {
            result = statement.eval(environment)?;
            if let Object::Return(_) = result {
                break;
            }
        }
        Ok(result)
    }
}

impl Expression {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        Ok(match self {
            Expression::Literal(literal) => match literal {
                Literal::Int(integer) => Object::Int(integer),
                Literal::True => Object::Bool(true),
                Literal::False => Object::Bool(false),
                Literal::String(string) => Object::String(string),
                Literal::Nil => Object::Nil,
            },
            //FIXME: remove clonnning
            Expression::Identifier(ident) => environment
                .store
                .get(&ident)
                .ok_or(anyhow!("Identifier {ident} not found"))?
                .to_owned(),
            Expression::Oposite(exp) => exp.0.eval(environment)?.oposite()?,
            Expression::Not(exp) => exp.0.eval(environment)?.not()?,
            Expression::Eq(exp) => exp
                .lhs
                .eval(environment)?
                .eq(exp.rhs.eval(environment)?)
                .into(),
            Expression::NotEq(exp) => exp
                .lhs
                .eval(environment)?
                .not_eq(exp.rhs.eval(environment)?)
                .into(),
            Expression::Lt(exp) => exp
                .lhs
                .eval(environment)?
                .lt(exp.rhs.eval(environment)?)
                .into(),
            Expression::Lte(exp) => exp
                .lhs
                .eval(environment)?
                .lte(exp.rhs.eval(environment)?)
                .into(),
            Expression::Gt(exp) => exp
                .lhs
                .eval(environment)?
                .gt(exp.rhs.eval(environment)?)
                .into(),
            Expression::Gte(exp) => exp
                .lhs
                .eval(environment)?
                .gte(exp.rhs.eval(environment)?)
                .into(),
            Expression::Add(exp) => exp.lhs.eval(environment)?.add(exp.rhs.eval(environment)?)?,
            Expression::Sub(exp) => exp.lhs.eval(environment)?.sub(exp.rhs.eval(environment)?)?,
            Expression::Mul(exp) => exp.lhs.eval(environment)?.mul(exp.rhs.eval(environment)?)?,
            Expression::Div(exp) => exp.lhs.eval(environment)?.div(exp.rhs.eval(environment)?)?,
            Expression::If(if_exp) => if_exp.eval(environment)?,
            Expression::Function(fn_exp) => fn_exp.eval(environment)?,
            Expression::Call(_) => todo!(),
        })
    }
}

impl FunctionExpression {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        Ok(Object::Function(Function::new(
            self.parameters,
            self.body,
            //FIXME: cloneee
            environment.clone(),
        )))
    }
}

impl IfExpression {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        let condition = self.condition.eval(environment)?.into_bool()?;

        Ok(if condition {
            self.consequence.eval(environment)?
        } else if let Some(alternative) = self.alternative {
            alternative.eval(environment)?
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
        let mut environment = Environment::new(None);

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
    fn eval_let_statements() {
        let input = [
            "let a = 5; a;",
            "let a = 5 * 5; a;",
            "let a = 5; let b = a; b;",
            "let a = 5; let b = a; let c = a + b + 5; c;",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_function_declaration() {
        let input = ["fn(x) { x + 2; };"];

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
            "foobar;",
        ];

        let result: Vec<String> = input
            .iter()
            .flat_map(|x| {
                let lexer = Lexer::new(x);
                let parser = Parser::new(lexer);
                let mut environment = Environment::new(None);
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
