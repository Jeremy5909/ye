use crate::parser::Parser;
use crate::{ast::Expr, error::ParsingError, scanner::token::Token};

impl Parser {
    pub fn parse_unary(&mut self) -> Result<Expr, ParsingError> {
        match self.peek() {
            Some(Token::Not) | Some(Token::Sub) => {
                let op = self.advance().unwrap().clone();
                let right = self.parse_call()?;
                Ok(Expr::Unary(op, Box::new(right)))
            }
            _ => self.parse_call(),
        }
    }
}
