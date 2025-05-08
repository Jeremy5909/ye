use crate::token::Token;

use super::{Expr, Value};

impl Expr {
    pub fn eval(&self) -> Value {
        match self {
            Expr::Variable(v) => todo!(),
            Expr::Literal(v) => v.clone(),
            Expr::Binary(left, op, right) => {
                let l = left.eval();
                let r = right.eval();
                match (l, r, op) {
                    (Value::Number(a), Value::Number(b), Token::Add) => Value::Number(a + b),
                    (Value::Number(a), Value::Number(b), Token::Sub) => Value::Number(a - b),
                    (Value::Number(a), Value::Number(b), Token::Mult) => Value::Number(a * b),
                    (Value::Number(a), Value::Number(b), Token::Div) => Value::Number(a / b),
                    (Value::Str(a), Value::Str(b), Token::Add) => Value::Str(a + b.as_str()),
                    _ => panic!("Invalid operands"),
                }
            }
        }
    }
}
