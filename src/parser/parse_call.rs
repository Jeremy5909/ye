use crate::parser::Parser;
use crate::{ast::Expr, error::ParsingError, scanner::token::Token};

impl Parser {
    pub fn parse_call(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.consume(Token::LParen).is_ok() {
                let mut args = Vec::new();
                if self.consume(Token::RParen).is_err() {
                    loop {
                        args.push(self.parse_expr()?);
                        if self.consume(Token::Comma).is_err() {
                            self.consume(Token::RParen)?;
                            break;
                        }
                    }
                }
                expr = Expr::Call(Box::new(expr), args)
            } else if self.consume(Token::LBracket).is_ok() {
                let index = self.parse_expr()?;
                self.consume(Token::RBracket)?;
                expr = Expr::Index(Box::new(expr), Box::new(index))
            } else {
                break;
            }
        }
        Ok(expr)
    }
}
