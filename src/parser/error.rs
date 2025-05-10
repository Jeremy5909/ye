use std::fmt::Debug;

pub enum ParsingError {
    VariableNotFound,
    InvalidOperands,
    UnexpectedEndOfInput,
    UncompletedParenthesis,
    UnexpectedToken,
    ExpectedToken,
}
impl Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::VariableNotFound => f.write_str("Variable not found"),
            ParsingError::InvalidOperands => f.write_str("Invalid operands"),
            _ => f.write_str("An error has occured"),
        }
    }
}
