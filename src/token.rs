#[derive(Debug, Clone)]
pub enum Token {
    Add,
    Sub,
    Mult,
    Div,
    Equal,
    Not,

    LParen,
    RParen,

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
