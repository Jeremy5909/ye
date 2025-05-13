use crate::{
    ast::{Expr, Value},
    environment::Environment,
    error::ParsingError,
    scanner::token::Token,
};

pub fn eval_unary(
    env: &mut Environment,
    op: &Token,
    expr: &Box<Expr>,
) -> Result<Value, ParsingError> {
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
