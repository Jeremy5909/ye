use crate::{
    ast::{Expr, Function, Value},
    environment::Environment,
    error::ParsingError,
};

pub fn eval_call(
    env: &mut Environment,
    expr: &Box<Expr>,
    args: &Vec<Expr>,
) -> Result<Value, ParsingError> {
    let func = expr.eval(env)?;
    let args: Result<Vec<_>, _> = args.iter().map(|arg| arg.eval(env)).collect();
    let args = args?;
    match func {
        Value::Function(Function { params, body }) => {
            if params.len() != args.len() {
                return Err(ParsingError::WrongNumArgs(args.len(), params.len()));
            }
            let mut local_env = env.new_child();
            for (param, value) in params.into_iter().zip(args) {
                local_env.set(&param, value);
            }
            let mut result = None;
            for stmt in body {
                result = stmt.eval(&mut local_env)?;
            }
            Ok(result.unwrap_or(Value::Bool(false)))
        }
        Value::NativeFunction(f) => f(args).map_err(ParsingError::NativeError),
        _ => Err(ParsingError::NotCallable),
    }
}
