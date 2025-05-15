use crate::{
    ast::{Expr, Value},
    error::ParsingError,
    scanner::token::Token,
};

use super::Parser;

impl Parser {
    pub fn parse_primary(&mut self) -> Result<Expr, ParsingError> {
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
                Token::Fn => {
                    self.consume(Token::LBracket)?;
                    let mut params = Vec::new();
                    if self.consume(Token::RBracket).is_err() {
                        loop {
                            params.push(self.consume_id()?);
                            if self.consume(Token::Comma).is_err() {
                                self.consume(Token::RBracket)?;
                                break;
                            }
                        }
                    }
                    self.consume(Token::LBrace)?;
                    let mut body = Vec::new();
                    while self.peek() != Some(&Token::RBrace) {
                        body.push(self.parse_statement()?);
                    }
                    self.consume(Token::RBrace)?;
                    Ok(Expr::Function(params, body))
                }
                Token::If => {
                    let condition = self.parse_expr()?;
                    self.consume(Token::LBrace)?;
                    let mut then_branch = Vec::new();
                    while self.peek() != Some(&Token::RBrace) {
                        then_branch.push(self.parse_statement()?);
                    }
                    self.consume(Token::RBrace)?;

                    let else_branch = if self.consume(Token::Else).is_ok() {
                        self.consume(Token::LBrace)?;
                        let mut else_branch = Vec::new();
                        while self.peek() != Some(&Token::RBrace) {
                            else_branch.push(self.parse_statement()?);
                        }
                        self.consume(Token::RBrace)?;
                        Some(else_branch)
                    } else {
                        None
                    };

                    Ok(Expr::If(Box::new(condition), then_branch, else_branch))
                }
                Token::While => {
                    let condition = self.parse_expr()?;
                    self.consume(Token::LBrace)?;
                    let mut exprs = Vec::new();
                    while self.peek() != Some(&Token::RBrace) {
                        exprs.push(self.parse_statement()?);
                    }
                    self.consume(Token::RBrace)?;
                    Ok(Expr::While(Box::new(condition), exprs))
                }
                Token::LBracket => {
                    let mut arr = Vec::new();
                    if self.consume(Token::RBracket).is_err() {
                        loop {
                            arr.push(self.parse_expr()?);
                            if self.consume(Token::Comma).is_err() {
                                self.consume(Token::RBracket)?;
                                break;
                            }
                        }
                    }
                    Ok(Expr::ArrayLiteral(arr))
                }
                _ => Err(ParsingError::UnexpectedToken(tok.clone())),
            }
        } else {
            Err(ParsingError::UnexpectedEndOfInput)
        }
    }
}
