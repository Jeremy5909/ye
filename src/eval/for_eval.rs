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

    let mut result = Vec::new();
    for item in elements {
        let mut loop_env = env.new_child();
        loop_env.set(&iter_name, item);

        for stmt in stmts {
            if let Some(val) = stmt.eval(&mut loop_env)? {
                match val {
                    // should this typa thing be handeled earlier? should void even be a value type
                    // or just option?
                    Value::Void => (),
                    _ => result.push(val),
                }
            }
        }
    }
    Ok(Value::Array(result))
}
