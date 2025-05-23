use crate::{
    ast::{Expr, Value},
    environment::Environment,
    error::ParsingError,
};

pub fn eval_if(
    env: &mut Environment,
    condition: &Expr,
    then_branch: &Vec<crate::ast::Statement>,
    else_branch: &Option<Vec<crate::ast::Statement>>,
) -> Result<Value, ParsingError> {
    let cond = condition.eval(env)?;
    match cond {
        Value::Bool(true) => {
            let mut result = None;
            for stmt in then_branch {
                result = stmt.eval(env)?;
            }
            Ok(result.unwrap_or(Value::Void))
        }
        Value::Bool(false) => {
            if let Some(else_branch) = else_branch {
                let mut result = None;
                for stmt in else_branch {
                    result = stmt.eval(env)?;
                }
                Ok(result.unwrap_or(Value::Void))
            } else {
                Ok(Value::Void)
            }
        }
        _ => Err(ParsingError::ExpectedBoolean),
    }
}
