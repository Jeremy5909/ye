use crate::{
    ast::{Expr, Value},
    environment::Environment,
    error::ParsingError,
};

pub fn eval_assign(env: &mut Environment, name: &str, expr: &Expr) -> Result<Value, ParsingError> {
    if !env.contains(name) {
        return Result::Err(ParsingError::VariableNotFound(name.to_owned()));
    }
    let value = expr.eval(env)?;
    env.set(name, value.clone());
    Ok(value)
}
