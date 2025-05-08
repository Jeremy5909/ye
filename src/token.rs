#[derive(Debug, Clone)]
pub enum Token {
    Add,
    Sub,
    Mult,
    Div,

    LParen,
    RParen,

    For,
    If,
    Fn,

    Str(String),
    Float(f32),

    Identifier(String),
    Unknown,
}
