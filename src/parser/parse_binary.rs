use crate::{ast::Expr, error::ParsingError, scanner::token::Token};

use super::Parser;

impl Parser {
    fn parse_binary<F>(&mut self, parse_next: F, valid_ops: &[Token]) -> Result<Expr, ParsingError>
    where
        F: Fn(&mut Self) -> Result<Expr, ParsingError>,
    {
        let mut expr = parse_next(self)?;
        while let Some(tok) = self.peek() {
            if valid_ops.contains(tok) {
                let op = self.advance().unwrap().clone();
                let right = parse_next(self)?;
                expr = Expr::Binary(Box::new(expr), op, Box::new(right));
            } else {
                break;
            }
        }
        Ok(expr)
    }

    pub fn parse_comparison(&mut self) -> Result<Expr, ParsingError> {
        self.parse_binary(
            Self::parse_equality,
            &[
                Token::Greater,
                Token::GreaterEqual,
                Token::Less,
                Token::LessEqual,
                Token::And,
                Token::Or,
            ],
        )
    }
    pub fn parse_equality(&mut self) -> Result<Expr, ParsingError> {
        self.parse_binary(Self::parse_term, &[Token::EqualEqual, Token::NotEqual])
    }
    pub fn parse_term(&mut self) -> Result<Expr, ParsingError> {
        self.parse_binary(Self::parse_factor, &[Token::Add, Token::Sub])
    }
    pub fn parse_factor(&mut self) -> Result<Expr, ParsingError> {
        self.parse_binary(Self::parse_unary, &[Token::Mult, Token::Div])
    }
}
