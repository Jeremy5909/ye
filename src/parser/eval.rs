use std::collections::HashMap;

use crate::token::Token;

use super::{Expr, Value};

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
    pub fn eval(&self, env: &mut Enviroment) -> Value {
        match self {
            Expr::Variable(v) => env
                .get(v)
                .unwrap_or_else(|| panic!("Undefined variable: {}", v))
                .clone(),
            Expr::Literal(v) => v.clone(),
            Expr::Binary(left, op, right) => {
                let l = left.eval(env);
                let r = right.eval(env);
                match (l, r, op) {
                    (Value::Number(a), Value::Number(b), Token::Add) => Value::Number(a + b),
                    (Value::Number(a), Value::Number(b), Token::Sub) => Value::Number(a - b),
                    (Value::Number(a), Value::Number(b), Token::Mult) => Value::Number(a * b),
                    (Value::Number(a), Value::Number(b), Token::Div) => Value::Number(a / b),
                    (Value::Str(a), Value::Str(b), Token::Add) => Value::Str(a + b.as_str()),
                    _ => panic!("Invalid operands"),
                }
            }
            Expr::Assign(name, expr) => {
                let value = expr.eval(env);
                env.set(name.to_owned(), value.clone());
                value
            }
        }
    }
}
