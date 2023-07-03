use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

use serde::Serialize;
use smol_str::SmolStr;
use thiserror::Error;

use crate::{
    ast::{BinaryOperator, BlockStatement, UnaryOperator},
    token::Identifier,
};

#[derive(Debug, Serialize, Clone)]
pub enum Object {
    Nil,
    Int(i64),
    Bool(bool),
    String(SmolStr),
    Function(Function),
    Return(Box<Object>),
}

type Result<T> = std::result::Result<T, EvalError>;

pub static TRUE: Object = Object::Bool(true);
pub static FALSE: Object = Object::Bool(false);
pub static NIL: Object = Object::Nil;
pub static ZERO: Object = Object::Int(0);
pub static EMPTY_STRING: Object = Object::String(SmolStr::new_inline(""));

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("operator `{operator:?}` not supported for value {operand:?}")]
    UnaryOpError {
        operator: UnaryOperator,
        operand: Object,
    },
    #[error("operator `{operator:?}` not supported between values {lhs:?} and {rhs:?}")]
    BinaryOpError {
        operator: BinaryOperator,
        lhs: Object,
        rhs: Object,
    },
    #[error("{target} coercion unsuported for {value:?}")]
    CoercionError { target: &'static str, value: Object },
    #[error("Identifier {0} already defined")]
    IdentifierAlreadyDefined(Identifier),
}

#[derive(Debug, Serialize, Clone)]
pub struct Environment {
    //maybe use a vec
    curr: HashMap<SmolStr, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            curr: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Self, curr: HashMap<SmolStr, Object>) -> Self {
        Self {
            curr,
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &Identifier) -> Option<Object> {
        self.curr
            .get(&name.inner())
            //FIXME: remove clone
            .cloned()
            .or_else(|| self.outer.as_ref().and_then(|x| x.get(name)))
    }

    pub fn try_insert(&mut self, ident: &Identifier, value: Object) -> Result<()> {
        match self.curr.entry(ident.inner()) {
            Entry::Occupied(_) => Err(EvalError::IdentifierAlreadyDefined(ident.clone())),
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .parameters
            .iter()
            .map(|x| x.inner())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "fn({}) {{todo()!}}", params)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "nil"),
            Object::Int(int) => write!(f, "{}", int),
            Object::Bool(bool) => write!(f, "{}", bool),
            Object::String(str) => write!(f, "{}", str),
            Object::Function(function) => write!(f, "{}", function),
            Object::Return(obj) => write!(f, "{}", obj),
        }
    }
}

impl Function {
    pub fn new(parameters: Vec<Identifier>, body: BlockStatement, env: Environment) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }
}

impl Object {
    pub fn as_typeof(&self) -> &'static str {
        match self {
            Object::Nil => "nil",
            Object::Int(_) => "int",
            Object::Bool(_) => "bool",
            Object::String(_) => "string",
            Object::Function(_) => "function",
            Object::Return(_) => "return",
        }
    }

    pub fn into_string(self) -> Result<SmolStr> {
        Ok(match self {
            Object::String(str) => str,
            Object::Int(int) => int.to_string().into(),
            Object::Bool(bool) => bool.to_string().into(),
            other => {
                return Err(EvalError::CoercionError {
                    target: EMPTY_STRING.as_typeof(),
                    value: other,
                })
            }
        })
    }

    pub fn into_int(self) -> Result<i64> {
        Ok(match self {
            Object::Int(int) => int,
            Object::Bool(bool) => bool.into(),
            Object::String(value) => match value.parse() {
                Ok(int) => int,
                Err(_) => Err(EvalError::CoercionError {
                    target: ZERO.as_typeof(),
                    value: Object::String(value),
                })?,
            },
            other => {
                return Err(EvalError::CoercionError {
                    target: ZERO.as_typeof(),
                    value: other,
                })
            }
        })
    }

    pub fn into_bool(self) -> Result<bool> {
        Ok(match self {
            Object::Nil => false,
            Object::Int(int) => int != 0,
            Object::Bool(bool) => bool,
            Object::String(str) => !str.is_empty(),
            Object::Function(_) => true,
            other => {
                return Err(EvalError::CoercionError {
                    target: TRUE.as_typeof(),
                    value: other,
                })
            }
        })
    }

    pub fn not(self) -> Result<Object> {
        Ok((!self.into_bool()?).into())
    }

    pub fn minus(self) -> Result<Object> {
        match self {
            Object::Int(int) => Ok(Object::Int(-int)),
            operand => {
                return Err(EvalError::UnaryOpError {
                    operator: UnaryOperator::Minus,
                    operand,
                })
            }
        }
    }

    pub fn add(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs + rhs),
            (Object::String(lhs), Object::String(rhs)) => {
                Object::String(format!("{}{}", lhs, rhs).into())
            }
            (lhs, rhs) => {
                return Err(EvalError::BinaryOpError {
                    operator: BinaryOperator::Add,
                    lhs,
                    rhs,
                })
            }
        })
    }

    pub fn sub(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs - rhs),
            (lhs, rhs) => {
                return Err(EvalError::BinaryOpError {
                    operator: BinaryOperator::Sub,
                    lhs,
                    rhs,
                })
            }
        })
    }

    pub fn mul(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs * rhs),
            (lhs, rhs) => {
                return Err(EvalError::BinaryOpError {
                    operator: BinaryOperator::Mul,
                    lhs,
                    rhs,
                })
            }
        })
    }

    pub fn div(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs / rhs),
            (lhs, rhs) => {
                return Err(EvalError::BinaryOpError {
                    operator: BinaryOperator::Div,
                    lhs,
                    rhs,
                })
            }
        })
    }

    pub fn eq(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs == rhs,
            (Object::String(lhs), Object::String(rhs)) => lhs == rhs,
            (Object::Bool(lhs), Object::Bool(rhs)) => lhs == rhs,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }

    pub fn not_eq(self, rhs: Object) -> bool {
        !self.eq(rhs)
    }

    pub fn lt(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs < rhs,
            _ => false,
        }
    }

    pub fn gt(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs > rhs,
            _ => false,
        }
    }

    pub fn lte(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs <= rhs,
            _ => false,
        }
    }

    pub fn gte(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs >= rhs,
            _ => false,
        }
    }
}

impl From<bool> for Object {
    fn from(bool: bool) -> Self {
        if bool {
            TRUE.clone()
        } else {
            FALSE.clone()
        }
    }
}
