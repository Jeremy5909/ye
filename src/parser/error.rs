use std::fmt::Debug;

use crate::token::Token;

pub enum ParsingError {
    VariableNotFound(String),
    InvalidOperands,
    UnexpectedEndOfInput,
    UncompletedParenthesis,
    UnexpectedToken(Token),
    ExpectedIdentifier,
    ExpectedToken(Token),
    ExpectedVariable,
}
impl Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::VariableNotFound(name) => write!(f, "Variable not found: {name}"),
            ParsingError::InvalidOperands => write!(f, "Invalid operands"),
            ParsingError::UnexpectedToken(tok) => write!(f, "Unexpected token: {tok:?}"),
            ParsingError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParsingError::ExpectedToken(tok) => write!(f, "Expected token: {tok:?}"),
            ParsingError::ExpectedIdentifier => write!(f, "Expected identifier"),
            ParsingError::UncompletedParenthesis => write!(f, "Uncompleted parenthesis"),
            _ => f.write_str("An error has occured"),
        }
    }
}
