#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Mult,
    Div,
    Equal,
    Not,
    NotEqual,

    EqualEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,

    For,
    If,
    Else,
    Fn,
    Let,
    Import,

    Str(String),
    Float(f32),
    Bool(bool),

    Identifier(String),
    Unknown,
}
