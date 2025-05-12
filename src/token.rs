#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Mult,
    Div,
    Equal,
    Not,

    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,

    For,
    If,
    Fn,
    Let,

    Str(String),
    Float(f32),
    Bool(bool),

    Identifier(String),
    Unknown,
}
