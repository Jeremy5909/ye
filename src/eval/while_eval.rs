use crate::{
    ast::{Expr, Statement, Value},
    environment::Environment,
    error::ParsingError,
};

pub fn eval_while(
    env: &mut Environment,
    condition: &Expr,
    stmts: &Vec<Statement>,
) -> Result<Value, ParsingError> {
    let mut result = None;
    loop {
        match condition.eval(env)? {
            Value::Bool(true) => {
                for stmt in stmts {
                    result = stmt.eval(env)?;
                }
            }
            Value::Bool(false) => break,
            _ => return Err(ParsingError::ExpectedBoolean),
        }
    }
    Ok(result.unwrap_or(Value::Void))
}
