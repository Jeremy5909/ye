use crate::parser::Parser;
use crate::{ast::Expr, error::ParsingError, scanner::token::Token};

impl Parser {
    pub fn parse_call(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.consume(Token::Colon).is_ok() {
                let args_expr = self.parse_expr()?;
                expr = Expr::Call(Box::new(expr), Box::new(args_expr))
            } else if self.consume(Token::LBracket).is_ok() {
                let index = self.parse_expr()?;
                self.consume(Token::RBracket)?;
                expr = Expr::Index(Box::new(expr), Box::new(index))
            } else if self.consume(Token::DoubleColon).is_ok() {
                let arg = self.consume_id()?;
                let block = self.consume_block()?;
                expr = Expr::For(Box::new(expr), arg, block)
            } else {
                break;
            }
        }
        Ok(expr)
    }
}
