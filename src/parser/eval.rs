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
                    (Value::Number(a), Value::Number(b), Token::Less) => Ok(Value::Bool(a < b)),
                    (Value::Bool(a), Value::Bool(b), Token::And) => Ok(Value::Bool(a & b)),
                    (Value::Bool(a), Value::Bool(b), Token::Or) => Ok(Value::Bool(a || b)),
                    (Value::Number(a), Value::Number(b), Token::LessEqual) => {
                        Ok(Value::Bool(a <= b))
                    }
                    (Value::Number(a), Value::Number(b), Token::Greater) => Ok(Value::Bool(a > b)),
                    (Value::Number(a), Value::Number(b), Token::GreaterEqual) => {
                        Ok(Value::Bool(a >= b))
                    }
                    (Value::Number(a), Value::Number(b), Token::EqualEqual) => {
                        Ok(Value::Bool(a == b))
                    }
                    (Value::Number(a), Value::Number(b), Token::NotEqual) => {
                        Ok(Value::Bool(a != b))
                    }
                    (Value::Str(a), Value::Str(b), Token::Add) => Ok(Value::Str(a + b.as_str())),
                    _ => Result::Err(ParsingError::InvalidOperands),
                }
            }
            Expr::Assign(name, expr) => {
                if !env.contains(name) {
                    return Result::Err(ParsingError::VariableNotFound(name.clone()));
                }
                let value = expr.eval(env)?;
                env.set(name, value.clone());
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
            Expr::If(condition, then_branch, else_branch) => {
                let cond = condition.eval(env)?;
                match cond {
                    Value::Bool(true) => {
                        let mut result = None;
                        for stmt in then_branch {
                            result = stmt.eval(env)?;
                        }
                        Ok(result.unwrap_or(Value::Bool(false)))
                    }
                    Value::Bool(false) => {
                        if let Some(else_branch) = else_branch {
                            let mut result = None;
                            for stmt in else_branch {
                                result = stmt.eval(env)?;
                            }
                            Ok(result.unwrap_or(Value::Bool(false)))
                        } else {
                            Ok(Value::Bool(false))
                        }
                    }
                    _ => Err(ParsingError::ExpectedBoolean),
                }
            }
            Expr::ArrayLiteral(elements) => {
                let mut arr = Vec::new();
                for element in elements {
                    arr.push(element.eval(env)?);
                }
                Ok(Value::Array(arr))
            }
            Expr::Index(arr, index) => {
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
                    _ => Err(ParsingError::NotIndexable(arr)),
                }
            }
        }
    }
}
