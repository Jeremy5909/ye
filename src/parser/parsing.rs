use crate::token::Token;

use super::{
    Parser,
    ast::{Expr, Statement, Value},
    error::ParsingError,
};

impl Parser {
    pub fn parse_all(&mut self) -> Result<Vec<Statement>, ParsingError> {
        let mut statements = Vec::new();
        while self.peek().is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }
    fn parse_binary<F: Fn(&mut Self) -> Result<Expr, ParsingError>>(
        &mut self,
        parse_next: F,
        valid_ops: &[Token],
    ) -> Result<Expr, ParsingError> {
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
    fn parse_statement(&mut self) -> Result<Statement, ParsingError> {
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
    fn parse_expr(&mut self) -> Result<Expr, ParsingError> {
        self.parse_assignment()
    }
    fn parse_assignment(&mut self) -> Result<Expr, ParsingError> {
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
    fn parse_comparison(&mut self) -> Result<Expr, ParsingError> {
        self.parse_binary(
            Self::parse_term,
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
    fn parse_term(&mut self) -> Result<Expr, ParsingError> {
        self.parse_binary(Self::parse_equality, &[Token::Add, Token::Sub])
    }
    fn parse_equality(&mut self) -> Result<Expr, ParsingError> {
        self.parse_binary(Self::parse_factor, &[Token::EqualEqual, Token::NotEqual])
    }
    fn parse_factor(&mut self) -> Result<Expr, ParsingError> {
        self.parse_binary(Self::parse_unary, &[Token::Mult, Token::Div])
    }
    fn parse_unary(&mut self) -> Result<Expr, ParsingError> {
        match self.peek() {
            Some(Token::Not) | Some(Token::Sub) => {
                let op = self.advance().unwrap().clone();
                let right = self.parse_call()?;
                Ok(Expr::Unary(op, Box::new(right)))
            }
            _ => self.parse_call(),
        }
    }
    fn parse_call(&mut self) -> Result<Expr, ParsingError> {
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
                Token::Fn => {
                    self.consume(Token::LParen)?;
                    let mut params = Vec::new();
                    if self.consume(Token::RParen).is_err() {
                        loop {
                            params.push(self.consume_id()?);
                            if self.consume(Token::Comma).is_err() {
                                self.consume(Token::RParen)?;
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
