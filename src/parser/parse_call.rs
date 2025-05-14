use crate::parser::Parser;
use crate::{ast::Expr, error::ParsingError, scanner::token::Token};

impl Parser {
    pub fn parse_call(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.consume(Token::Colon).is_ok() {
                let args_expr = self.parse_expr()?;
                expr = Expr::Call(Box::new(expr), Box::new(args_expr))
            } else if self.consume(Token::LBrace).is_ok() {
                let index = self.parse_expr()?;
                self.consume(Token::RBrace)?;
                expr = Expr::Index(Box::new(expr), Box::new(index))
            } else {
                break;
            }
        }
        Ok(expr)
    }
}
