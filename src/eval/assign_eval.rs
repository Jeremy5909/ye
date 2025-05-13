use crate::{
    ast::{Expr, Value},
    environment::Environment,
    error::ParsingError,
};

pub fn eval_assign(
    env: &mut Environment,
    name: &String,
    expr: &Box<Expr>,
) -> Result<Value, ParsingError> {
    if !env.contains(name) {
        return Result::Err(ParsingError::VariableNotFound(name.clone()));
    }
    let value = expr.eval(env)?;
    env.set(name, value.clone());
    Ok(value)
}
