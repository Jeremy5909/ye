use crate::token::Token;

use super::{
    ast::{Expr, Function, Value},
    environment::Environment,
    error::ParsingError,
};

impl Expr {
    pub fn eval(&self, env: &mut Environment) -> Result<Value, ParsingError> {
        match self {
            Expr::Variable(v) => env
                .get(v)
                .cloned()
                .ok_or(ParsingError::VariableNotFound(v.clone())),
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Binary(left, op, right) => {
                let l = left.eval(env)?;
                let r = right.eval(env)?;
                match (l, r, op) {
                    (Value::Number(a), Value::Number(b), Token::Add) => Ok(Value::Number(a + b)),
                    (Value::Number(a), Value::Number(b), Token::Sub) => Ok(Value::Number(a - b)),
                    (Value::Number(a), Value::Number(b), Token::Mult) => Ok(Value::Number(a * b)),
                    (Value::Number(a), Value::Number(b), Token::Div) => Ok(Value::Number(a / b)),
                    (Value::Str(a), Value::Str(b), Token::Add) => Ok(Value::Str(a + b.as_str())),
                    _ => Result::Err(ParsingError::InvalidOperands),
                }
            }
            Expr::Assign(name, expr) => {
                if !env.contains(name) {
                    return Result::Err(ParsingError::VariableNotFound(name.clone()));
                }
                let value = expr.eval(env)?;
                env.set(name.to_owned(), value.clone());
                Ok(value)
            }
            Expr::Unary(op, expr) => {
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
            Expr::Function(params, body) => Ok(Value::Function(Function {
                params: params.clone(),
                body: body.clone(),
            })),
            Expr::Call(expr, args) => {
                let func = expr.eval(env)?;
                let arg_values = args
                    .iter()
                    .map(|arg| arg.eval(env))
                    .collect::<Result<Vec<_>, _>>()?;
                match func {
                    Value::Function(Function { params, body }) => {
                        if params.len() != arg_values.len() {
                            return Err(ParsingError::WrongNumArgs(arg_values.len(), params.len()));
                        }
                        let mut local_env = env.new_child();
                        for (param, value) in params.into_iter().zip(arg_values) {
                            local_env.set(param, value);
                        }
                        let mut result = None;
                        for stmt in body {
                            result = stmt.eval(&mut local_env)?;
                        }
                        Ok(result.unwrap_or(Value::Bool(false)))
                    }
                    _ => Err(ParsingError::NotCallable),
                }
            }
        }
    }
}
