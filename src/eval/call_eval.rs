use crate::{
    ast::{Expr, Function, Value},
    environment::Environment,
    error::ParsingError,
};

pub fn eval_call(
    env: &mut Environment,
    expr: &Expr,
    args_expr: &Expr,
) -> Result<Value, ParsingError> {
    let func = expr.eval(env)?;
    let arg_value = args_expr.eval(env)?;

    match func {
        Value::Function(Function { params, body }) => {
            let args = match arg_value {
                Value::Array(values) => values,
                other => vec![other],
            };
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
        Value::NativeFunction(f) => {
            let args = match arg_value {
                Value::Array(values) => values,
                other => vec![other],
            };
            f(args).map_err(ParsingError::NativeError)
        }
        Value::Array(arr) => {
            let index = match arg_value {
                // i need to make ints
                Value::Number(n) if (n.fract() == 0.0) => n as usize,
                _ => return Err(ParsingError::ExpectedInteger),
            };
            arr.get(index)
                .cloned()
                .ok_or(ParsingError::IndexOutOfBounds(index))
        }
        _ => Err(ParsingError::NotCallable),
    }
}
