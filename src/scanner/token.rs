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
    And,
    Or,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
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
