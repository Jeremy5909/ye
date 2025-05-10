use std::collections::HashMap;

use crate::token::Token;

use super::{Expr, Value, error::ParsingError};

#[derive(Default)]
pub struct Enviroment(HashMap<String, Value>);
impl Enviroment {
    pub fn new() -> Self {
        Self::default()
    }
    fn get(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }
    pub fn set(&mut self, name: String, value: Value) {
        self.0.insert(name, value);
    }
}

impl Expr {
    pub fn eval(&self, env: &mut Enviroment) -> Result<Value, ParsingError> {
        match self {
            Expr::Variable(v) => Ok(env
                .get(v)
                .unwrap_or_else(|| panic!("Undefined variable: {}", v))
                .clone()),
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Binary(left, op, right) => {
                let l = left.eval(env);
                let r = right.eval(env);
                match (l?, r?, op) {
                    (Value::Number(a), Value::Number(b), Token::Add) => Ok(Value::Number(a + b)),
                    (Value::Number(a), Value::Number(b), Token::Sub) => Ok(Value::Number(a - b)),
                    (Value::Number(a), Value::Number(b), Token::Mult) => Ok(Value::Number(a * b)),
                    (Value::Number(a), Value::Number(b), Token::Div) => Ok(Value::Number(a / b)),
                    (Value::Str(a), Value::Str(b), Token::Add) => Ok(Value::Str(a + b.as_str())),
                    _ => Result::Err(ParsingError::InvalidOperands),
                }
            }
            Expr::Assign(name, expr) => {
                if !env.0.contains_key(name) {
                    return Result::Err(ParsingError::VariableNotFound);
                }

                let value = expr.eval(env)?;
                env.set(name.to_owned(), value.clone());
                Ok(value)
            }
        }
    }
}
