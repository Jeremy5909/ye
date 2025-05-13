use crate::{
    ast::{Expr, Value},
    environment::Environment,
    error::ParsingError,
};

pub fn eval_index(
    env: &mut Environment,
    arr: &Box<Expr>,
    index: &Box<Expr>,
) -> Result<Value, ParsingError> {
    let arr = arr.eval(env)?;
    let index = index.eval(env)?;
    let i = match index {
        Value::Number(ind) => Ok(ind),
        _ => return Err(ParsingError::NotIndex(index)),
    }? as usize;
    match arr {
        Value::Array(elements) => elements
            .get(i)
            .cloned()
            .ok_or(ParsingError::IndexOutOfBounds(i)),
        Value::Str(string) => string
            .chars()
            .map(|char| Value::Str(char.to_string()))
            .nth(i)
            .ok_or(ParsingError::IndexOutOfBounds(i)),
        _ => Err(ParsingError::NotIndexable(arr)),
    }
}
