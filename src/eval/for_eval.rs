use crate::{
    ast::{Expr, Statement, Value},
    environment::Environment,
    error::ParsingError,
};

pub fn eval_for(
    env: &mut Environment,
    arr: &Expr,
    iter_name: String,
    stmts: &Vec<Statement>,
) -> Result<Value, ParsingError> {
    let elements = match arr.eval(env)? {
        Value::Array(arr) => arr,
        _ => return Err(ParsingError::ExpectedArray),
    };
    let mut result = Value::Void;
    for item in elements {
        let mut loop_env = env.new_child();
        loop_env.set(&iter_name, item);

        for stmt in stmts {
            if let Some(val) = stmt.eval(&mut loop_env)? {
                result = val;
            }
        }
    }
    Ok(result)
}
