use std::collections::HashMap;

use token::Token;

mod scan_core;
mod scan_types;
pub mod token;

pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    index: usize,
    keyword_map: HashMap<&'static str, Token>,
}

impl Scanner {
    pub fn from(source: &str) -> Self {
        let keyword_map = HashMap::from([
            ("for", Token::For),
            ("fn", Token::Fn),
            ("if", Token::If),
            ("let", Token::Let),
            ("true", Token::Bool(true)),
            ("false", Token::Bool(false)),
            ("import", Token::Import),
            ("else", Token::Else),
            ("or", Token::Or),
            ("and", Token::And),
            ("while", Token::While),
        ]);
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            index: 0,
            keyword_map,
        }
    }
}
