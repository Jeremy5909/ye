use crate::token::Token;

use super::Scanner;

impl Scanner {
    pub(super) fn scan_string(&mut self) -> Result<Token, String> {
        let mut string = String::new();
        loop {
            let next = self.advance().ok_or("Uncompleted thing")?;
            if next == '"' {
                return Ok(Token::Str(string));
            }
            string.push(next);
        }
    }
    pub(super) fn scan_identifier(&mut self, curr: char) -> Token {
        let mut keyword = String::from(curr);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                keyword.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.keyword_map
            .get(keyword.as_str())
            .cloned()
            .unwrap_or(Token::Identifier(keyword))
    }
    pub(super) fn scan_num(&mut self, curr: char) -> Token {
        let mut num_str = String::from(curr);

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        Token::Float(num_str.parse().unwrap())
    }
}
