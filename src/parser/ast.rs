use crate::Environment;
use crate::parser::ParsingError;
use crate::token::Token;

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
        }
    }
}
