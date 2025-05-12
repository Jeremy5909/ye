use std::fmt;

use crate::parser::ParsingError;
use crate::token::Token;
use crate::{Environment, inp_handling};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Assign(String, Box<Expr>),
    Function(Vec<String>, Vec<Statement>),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f32),
    Str(String),
    Bool(bool),
    Function(Function),
    NativeFunction(fn(Vec<Value>) -> Result<Value, String>),
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}",),
            Self::Str(s) => write!(f, "{s}"),
            Self::Bool(b) => write!(f, "{b}"),
            _ => write!(f, "?"),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(String, Expr),
    Expr(Expr),
    Import(String),
}
impl Statement {
    pub fn eval(&self, env: &mut Environment) -> Result<Option<Value>, ParsingError> {
        match self {
            Statement::Let(name, expr) => {
                let value = expr.eval(env)?;
                env.set(name.clone(), value);
                Ok(None)
            }
            Statement::Expr(expr) => expr.eval(env).map(Some),
            Statement::Import(path) => {
                let code = std::fs::read_to_string(path)
                    .map_err(|_| ParsingError::FileNotFound(path.to_owned()))?;
                inp_handling::run(code, env, false);
                Ok(None)
            }
        }
    }
}
