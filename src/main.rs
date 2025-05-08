use std::io::{Write, stdin, stdout};

use scanner::Scanner;

mod scanner;
mod token;

fn main() {
    loop {
        let mut inp = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut inp).unwrap();
        let mut scanner = Scanner::new(inp.trim());
        scanner.scan_tokens();
        println!("{:?}", scanner.tokens);
    }
}
