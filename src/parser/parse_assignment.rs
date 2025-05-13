use crate::{ast::Expr, error::ParsingError, scanner::token::Token};

use super::Parser;

impl Parser {
    pub fn parse_assignment(&mut self) -> Result<Expr, ParsingError> {
        let expr = self.parse_comparison()?;
        if self.consume(Token::Equal).is_err() {
            return Ok(expr);
        }
        if let Expr::Variable(name) = expr {
            let value_expr = self.parse_assignment()?;
            Ok(Expr::Assign(name, Box::new(value_expr)))
        } else {
            Err(ParsingError::ExpectedVariable)
        }
    }
}
