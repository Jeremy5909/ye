use std::collections::HashMap;

use crate::token::Token;

use super::{Expr, Value, error::ParsingError};

#[derive(Default)]
pub struct Environment(HashMap<String, Value>);
impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
    fn get(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }
    pub fn set(&mut self, name: String, value: Value) {
        self.0.insert(name, value);
    }
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }
}

impl Expr {
    pub fn eval(&self, env: &mut Environment) -> Result<Value, ParsingError> {
        match self {
            Expr::Variable(v) => env
                .get(v)
                .cloned()
                .ok_or(ParsingError::VariableNotFound(v.clone())),
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Binary(left, op, right) => {
                let l = left.eval(env)?;
                let r = right.eval(env)?;
                match (l, r, op) {
                    (Value::Number(a), Value::Number(b), Token::Add) => Ok(Value::Number(a + b)),
                    (Value::Number(a), Value::Number(b), Token::Sub) => Ok(Value::Number(a - b)),
                    (Value::Number(a), Value::Number(b), Token::Mult) => Ok(Value::Number(a * b)),
                    (Value::Number(a), Value::Number(b), Token::Div) => Ok(Value::Number(a / b)),
                    (Value::Str(a), Value::Str(b), Token::Add) => Ok(Value::Str(a + b.as_str())),
                    _ => Result::Err(ParsingError::InvalidOperands),
                }
            }
            Expr::Assign(name, expr) => {
                if !env.contains(name) {
                    return Result::Err(ParsingError::VariableNotFound(name.clone()));
                }
                let value = expr.eval(env)?;
                env.set(name.to_owned(), value.clone());
                Ok(value)
            }
            Expr::Unary(op, expr) => {
                let value = expr.eval(env)?;
                match op {
                    Token::Not => match value {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        _ => Err(ParsingError::InvalidOperands),
                    },
                    Token::Sub => match value {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(ParsingError::InvalidOperands),
                    },
                    _ => Err(ParsingError::InvalidOperands),
                }
            }
        }
    }
}
