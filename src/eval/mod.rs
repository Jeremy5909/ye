use assign_eval::eval_assign;
use binary_eval::eval_binary;
use call_eval::eval_call;
use if_eval::eval_if;
use index_eval::eval_index;
use unary_eval::eval_unary;

use super::{
    ast::{Expr, Function, Value},
    environment::Environment,
    error::ParsingError,
};

mod assign_eval;
mod binary_eval;
mod call_eval;
mod if_eval;
mod index_eval;
mod unary_eval;

impl Expr {
    pub fn eval(&self, env: &mut Environment) -> Result<Value, ParsingError> {
        match self {
            Expr::Variable(v) => env
                .get(v)
                .cloned()
                .ok_or(ParsingError::VariableNotFound(v.clone())),
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Binary(left, op, right) => eval_binary(env, left, op, right),
            Expr::Assign(name, expr) => eval_assign(env, name, expr),
            Expr::Unary(op, expr) => eval_unary(env, op, expr),
            Expr::Function(params, body) => Ok(Value::Function(Function {
                params: params.clone(),
                body: body.clone(),
            })),
            Expr::Call(expr, args) => eval_call(env, expr, args),
            Expr::If(condition, then_branch, else_branch) => {
                eval_if(env, condition, then_branch, else_branch)
            }
            Expr::ArrayLiteral(elements) => {
                let mut arr = Vec::new();
                for element in elements {
                    arr.push(element.eval(env)?);
                }
                Ok(Value::Array(arr))
            }
            Expr::Index(arr, index) => eval_index(env, arr, index),
        }
    }
}
