use crate::{
    ast::{Expr, Value},
    environment::Environment,
    error::ParsingError,
    scanner::token::Token,
};

pub fn eval_binary(
    env: &mut Environment,
    left: &Box<Expr>,
    op: &Token,
    right: &Box<Expr>,
) -> Result<Value, ParsingError> {
    let l = left.eval(env)?;
    let r = right.eval(env)?;
    match (l, r, op) {
        (Value::Number(a), Value::Number(b), Token::Add) => Ok(Value::Number(a + b)),
        (Value::Number(a), Value::Number(b), Token::Sub) => Ok(Value::Number(a - b)),
        (Value::Number(a), Value::Number(b), Token::Mult) => Ok(Value::Number(a * b)),
        (Value::Number(a), Value::Number(b), Token::Div) => Ok(Value::Number(a / b)),
        (Value::Number(a), Value::Number(b), Token::Less) => Ok(Value::Bool(a < b)),
        (Value::Bool(a), Value::Bool(b), Token::And) => Ok(Value::Bool(a & b)),
        (Value::Bool(a), Value::Bool(b), Token::Or) => Ok(Value::Bool(a || b)),
        (Value::Number(a), Value::Number(b), Token::LessEqual) => Ok(Value::Bool(a <= b)),
        (Value::Number(a), Value::Number(b), Token::Greater) => Ok(Value::Bool(a > b)),
        (Value::Number(a), Value::Number(b), Token::GreaterEqual) => Ok(Value::Bool(a >= b)),
        (Value::Number(a), Value::Number(b), Token::EqualEqual) => Ok(Value::Bool(a == b)),
        (Value::Number(a), Value::Number(b), Token::NotEqual) => Ok(Value::Bool(a != b)),
        (Value::Str(a), Value::Str(b), Token::Add) => Ok(Value::Str(a + b.as_str())),
        _ => Result::Err(ParsingError::InvalidOperands),
    }
}
