use crate::{
    ast::{Expr, Statement},
    error::ParsingError,
    scanner::token::Token,
};

mod parse_assignment;
mod parse_binary;
mod parse_call;
mod parse_primary;
mod parse_statement;
mod parse_unary;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }
    pub fn advance(&mut self) -> Option<&Token> {
        let c = self.tokens.get(self.index)?;
        self.index += 1;
        Some(c)
    }
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
    pub fn consume(&mut self, expected: Token) -> Result<(), ParsingError> {
        match self.peek() {
            Some(tok) if *tok == expected => {
                self.advance();
                Ok(())
            }
            _ => Err(ParsingError::ExpectedToken(expected)),
        }
    }
    pub fn consume_id(&mut self) -> Result<String, ParsingError> {
        match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(ParsingError::ExpectedIdentifier),
        }
    }
    pub fn consume_string(&mut self) -> Result<String, ParsingError> {
        match self.peek() {
            Some(Token::Str(string)) => {
                let string = string.clone();
                self.advance();
                Ok(string)
            }
            _ => Err(ParsingError::ExpectedString),
        }
    }

    pub fn parse_all(&mut self) -> Result<Vec<Statement>, ParsingError> {
        let mut statements = Vec::new();
        while self.peek().is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }
    fn parse_expr(&mut self) -> Result<Expr, ParsingError> {
        self.parse_assignment()
    }
}
