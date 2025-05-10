use crate::Parser;
use crate::parser::Expr;
use crate::token::Token;

use super::{Statement, Value, error::ParsingError};

impl Parser {
    pub fn parse(&mut self) -> Result<Statement, ParsingError> {
        self.parse_statement()
    }
    fn parse_statement(&mut self) -> Result<Statement, ParsingError> {
        if let Some(stmt) = self.peek() {
            match stmt {
                Token::Let => {
                    self.advance();
                    let name = if let Some(Token::Identifier(name)) = self.peek() {
                        name.clone()
                    } else {
                        return Err(ParsingError::ExpectedIdentifier);
                    };
                    self.advance();
                    if let Some(Token::Equal) = self.peek() {
                        self.advance();
                        let expr = self.parse_expr()?;
                        Ok(Statement::Let(name, expr))
                    } else {
                        Err(ParsingError::ExpectedToken(Token::Equal))
                    }
                }
                _ => Ok(Statement::Expr(self.parse_expr()?)),
            }
        } else {
            Ok(Statement::Expr(self.parse_expr()?))
        }
    }
    fn parse_expr(&mut self) -> Result<Expr, ParsingError> {
        self.parse_assignment()
    }
    fn parse_assignment(&mut self) -> Result<Expr, ParsingError> {
        let expr = self.parse_term()?;
        if let Some(Token::Equal) = self.peek() {
            self.advance();
            if let Expr::Variable(name) = expr {
                let value_expr = self.parse_assignment()?;
                Ok(Expr::Assign(name, Box::new(value_expr)))
            } else {
                Err(ParsingError::UnexpectedToken(Token::Equal))
            }
        } else {
            Ok(expr)
        }
    }
    fn parse_term(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.parse_factor()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Add | Token::Sub => {
                    let op = self.advance().unwrap().clone();
                    let right = self.parse_factor()?;
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }
    fn parse_factor(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.parse_unary()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Mult | Token::Div => {
                    let op = self.advance().unwrap().clone();
                    let right = self.parse_primary()?;
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }
    fn parse_unary(&mut self) -> Result<Expr, ParsingError> {
        match self.peek() {
            Some(Token::Exclamation) | Some(Token::Sub) => {
                let op = self.advance().unwrap().clone();
                let right = self.parse_unary()?;
                Ok(Expr::Unary(op, Box::new(right)))
            }
            _ => self.parse_primary(),
        }
    }
    fn parse_primary(&mut self) -> Result<Expr, ParsingError> {
        if let Some(tok) = self.advance() {
            match tok {
                Token::Float(n) => Ok(Expr::Literal(Value::Number(*n))),
                Token::Str(s) => Ok(Expr::Literal(Value::Str(s.clone()))),
                Token::Bool(b) => Ok(Expr::Literal(Value::Bool(*b))),
                Token::Identifier(name) => Ok(Expr::Variable(name.clone())),
                Token::LParen => {
                    let expr = self.parse_expr()?;
                    match self.advance() {
                        Some(Token::RParen) => Ok(expr),
                        _ => Err(ParsingError::UncompletedParenthesis),
                    }
                }
                _ => Err(ParsingError::UnexpectedToken(tok.clone())),
            }
        } else {
            Err(ParsingError::UnexpectedEndOfInput)
        }
    }
}
