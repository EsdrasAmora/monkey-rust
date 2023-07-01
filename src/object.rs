use std::collections::{hash_map::Entry, HashMap};

use anyhow::{bail, Error, Result};
use serde::Serialize;
use smol_str::SmolStr;

use crate::{
    ast::{self, Expression},
    token::Identifier,
};

#[derive(Debug, Serialize, Clone)]
pub enum Object {
    Nil,
    Int(i64),
    Bool(bool),
    String(SmolStr),
    // Error(String),
    Function(Function),
    Return(Box<Object>),
}

pub const TRUE: Object = Object::Bool(true);
pub const FALSE: Object = Object::Bool(false);
pub const NIL: Object = Object::Nil;

//TODO: create error type for this
fn unary_op_not_supported(op_type: &str, lhs: &Object) -> anyhow::Error {
    anyhow::anyhow!("operator {} not supported for type {:?}", op_type, lhs)
}

//TODO: create error type for this
fn binary_op_not_supported(op_type: &str, lhs: &Object, rhs: &Object) -> Error {
    anyhow::anyhow!(
        "operator {} not supported between types {:?} and {:?}",
        op_type,
        lhs,
        rhs
    )
}

impl Expression {
    #[inline]
    pub fn eval_binary_expression(&self) -> () {}
}

#[derive(Debug, Serialize, Clone)]
pub struct Environment {
    //TODO: use a faster hashmap
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
            .cloned()
            .or_else(|| self.outer.as_ref().and_then(|x| x.get(name)))
    }

    pub fn try_insert(&mut self, ident: &Identifier, value: Object) -> Result<()> {
        match self.curr.entry(ident.inner()) {
            Entry::Occupied(_) => bail!("Identifier {ident} already defined"),
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
    pub body: ast::BlockStatement,
    pub env: Environment,
    //global_uniqueFuncIdent_varname
}

impl Function {
    pub fn new(parameters: Vec<Identifier>, body: ast::BlockStatement, env: Environment) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }
}

impl Object {
    pub fn into_bool(self) -> Result<bool> {
        Ok(match self {
            Object::Nil => false,
            Object::Int(int) => int != 0,
            Object::Bool(bool) => bool,
            Object::String(str) => !str.is_empty(),
            Object::Function(_) => true,
            other => bail!("boolean coercion unsuported for type {:?}", &other),
        })
    }

    pub fn not(self) -> Result<Object> {
        Ok((!self.into_bool()?).into())
    }

    pub fn oposite(self) -> Result<Object> {
        match self {
            Object::Int(int) => Ok(Object::Int(-int)),
            obj => Err(unary_op_not_supported("oposite", &obj)),
        }
    }

    pub fn add(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs + rhs),
            (Object::String(lhs), Object::String(rhs)) => {
                Object::String(format!("{}{}", lhs, rhs).into())
            }
            (lhs, rhs) => return Err(binary_op_not_supported("add", &lhs, &rhs)),
        })
    }

    pub fn sub(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs - rhs),
            (lhs, rhs) => return Err(binary_op_not_supported("sub", &lhs, &rhs)),
        })
    }

    pub fn mul(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs * rhs),
            (lhs, rhs) => return Err(binary_op_not_supported("mul", &lhs, &rhs)),
        })
    }

    pub fn div(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs / rhs),
            (lhs, rhs) => return Err(binary_op_not_supported("div", &lhs, &rhs)),
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
            TRUE
        } else {
            FALSE
        }
    }
}
