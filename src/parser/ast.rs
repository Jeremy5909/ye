use std::fmt;

use crate::parser::ParsingError;
use crate::token::Token;
use crate::{Environment, inp_handling};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Value),                                        // 3.0
    Variable(String),                                      // x
    Binary(Box<Expr>, Token, Box<Expr>),                   // x+3.0
    Unary(Token, Box<Expr>),                               // !true
    Assign(String, Box<Expr>),                             // x=3
    Function(Vec<String>, Vec<Statement>),                 // fn(x) {x}
    Call(Box<Expr>, Vec<Expr>),                            // f()
    If(Box<Expr>, Vec<Statement>, Option<Vec<Statement>>), // if (x) {..} else {..}
    ArrayLiteral(Vec<Expr>),                               // [3, 5.0, true, "hi"]
    Index(Box<Expr>, Box<Expr>),                           // arr[3]
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f32),
    Str(String),
    Bool(bool),
    Function(Function),
    NativeFunction(fn(Vec<Value>) -> Result<Value, String>),
    Array(Vec<Value>),
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}",),
            Self::Str(s) => write!(f, "{s}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Array(v) => write!(f, "{v:?}"),
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
                env.set(name, value);
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
