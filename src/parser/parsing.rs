use crate::Parser;
use crate::parser::Expr;
use crate::token::Token;

use super::Value;

impl Parser {
    pub fn parse_expr(&mut self) -> Expr {
        self.parse_assignment()
    }
    fn parse_assignment(&mut self) -> Expr {
        let expr = self.parse_term();
        if let Some(Token::Equal) = self.peek() {
            self.advance();
            if let Expr::Variable(name) = expr {
                let value_expr = self.parse_assignment();
                return Expr::Assign(name, Box::new(value_expr));
            } else {
                panic!("Invalid assignment");
            }
        }
        expr
    }
    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        while let Some(tok) = self.peek() {
            match tok {
                Token::Add | Token::Sub => {
                    let op = self.advance().unwrap().clone();
                    let right = self.parse_factor();
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        expr
    }
    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        while let Some(tok) = self.peek() {
            match tok {
                Token::Mult | Token::Div => {
                    let op = self.advance().unwrap().clone();
                    let right = self.parse_primary();
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        expr
    }
    fn parse_primary(&mut self) -> Expr {
        if let Some(tok) = self.advance() {
            match tok {
                Token::Float(n) => Expr::Literal(Value::Number(*n)),
                Token::Str(s) => Expr::Literal(Value::Str(s.clone())),
                Token::Bool(b) => Expr::Literal(Value::Bool(*b)),
                Token::Identifier(name) => Expr::Variable(name.clone()),
                Token::LParen => {
                    let expr = self.parse_expr();
                    match self.advance() {
                        Some(Token::RParen) => expr,
                        _ => panic!("Expected ')'"),
                    }
                }
                _ => panic!("Unexpected token: {:?}", tok),
            }
        } else {
            panic!("Unexpected end of input");
        }
    }
}
