use crate::token::Token;
mod parsing;

#[derive(Debug)]
pub enum Expr {
    Literal(f32),
    Binary(Box<Expr>, Token, Box<Expr>),
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
