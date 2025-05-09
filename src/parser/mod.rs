use eval::Enviroment;

use crate::token::Token;
pub(crate) mod eval;
mod parsing;

#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Binary(Box<Expr>, Token, Box<Expr>),
    Assign(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expr),
    Expr(Expr),
}
impl Statement {
    pub fn eval(&self, env: &mut Enviroment) -> Option<Value> {
        match self {
            Statement::Let(name, expr) => {
                let value = expr.eval(env);
                env.set(name.clone(), value);
                None
            }
            Statement::Expr(expr) => Some(expr.eval(env)),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }
    fn advance(&mut self) -> Option<&Token> {
        let c = self.tokens.get(self.index)?;
        self.index += 1;
        Some(c)
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
}
