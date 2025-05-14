use super::{Scanner, token::Token};

impl Scanner {
    pub fn scan_tokens(&mut self) {
        while let Some(next) = self.advance() {
            if next.is_whitespace() {
                continue;
            }
            let token = match next {
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Mult,
                '/' => Token::Div,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                '|' => Token::Or,
                '&' => Token::And,
                ':' => Token::Colon,
                '=' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::EqualEqual
                    }
                    _ => Token::Equal,
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::GreaterEqual
                    }
                    _ => Token::Greater,
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::LessEqual
                    }
                    _ => Token::Less,
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::NotEqual
                    }
                    _ => Token::Not,
                },
                '"' => self.scan_string().unwrap(),
                _ => {
                    if next.is_numeric() || next == '.' {
                        self.scan_num(next)
                    } else if next.is_ascii_alphabetic() {
                        self.scan_identifier(next)
                    } else {
                        Token::Unknown
                    }
                }
            };
            self.tokens.push(token);
        }
    }
    pub(super) fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.index += 1;
        Some(c)
    }
    pub(super) fn peek(&self) -> Option<char> {
        self.source.get(self.index).cloned()
    }
}
