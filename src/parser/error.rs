use std::fmt::Debug;

use crate::token::Token;

use super::ast::Value;

pub enum ParsingError {
    VariableNotFound(String),
    InvalidOperands,
    UnexpectedEndOfInput,
    UncompletedParenthesis,
    UnexpectedToken(Token),
    ExpectedIdentifier,
    ExpectedString,
    ExpectedToken(Token),
    ExpectedVariable,
    ExpectedBoolean,
    NotCallable,
    WrongNumArgs(usize, usize),
    NativeError(String),
    FileNotFound(String),
    NotIndexable(Value),
    NotIndex(Value),
    IndexOutOfBounds(usize),
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
            ParsingError::ExpectedString => write!(f, "Expected string"),
            ParsingError::UncompletedParenthesis => write!(f, "Uncompleted parenthesis"),
            ParsingError::NotCallable => write!(f, "That is not callable"),
            ParsingError::WrongNumArgs(num_args, num_parameters) => write!(
                f,
                "That needs {num_parameters} parameters but you provided {num_args}"
            ),
            ParsingError::ExpectedVariable => write!(f, "Variable expected"),
            ParsingError::NativeError(e) => write!(f, "Native error: '{e}'"),
            ParsingError::FileNotFound(path) => write!(f, "File '{path}' not found"),
            ParsingError::ExpectedBoolean => write!(f, "Expected boolean"),
            ParsingError::NotIndexable(v) => write!(f, "You cannot index '{v}'"),
            ParsingError::NotIndex(v) => write!(f, "You cannot use {v} as an index"),
            ParsingError::IndexOutOfBounds(i) => write!(f, "Index '{i}' out of bounds"),
        }
    }
}
