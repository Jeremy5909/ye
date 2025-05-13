use crate::{error::ParsingError, parser::Statement, scanner::token::Token};

use super::Parser;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, ParsingError> {
        if self.consume(Token::Import).is_ok() {
            let path = self.consume_string()?;
            return Ok(Statement::Import(path.clone()));
        }
        if self.consume(Token::Let).is_err() {
            return Ok(Statement::Expr(self.parse_expr()?));
        }

        let name = self.consume_id()?;
        self.consume(Token::Equal)?;
        let expr = self.parse_expr()?;
        Ok(Statement::Let(name, expr))
    }
}
