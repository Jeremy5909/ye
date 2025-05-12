use error::ParsingError;

use crate::token::Token;
pub mod ast;
pub mod environment;
mod error;
pub mod eval;
mod parsing;

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
}
