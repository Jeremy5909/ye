use std::io::{Write, stdin, stdout};

use parser::Parser;
use scanner::Scanner;

mod parser;
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
        println!("Tokens:\n{:?}\n\n", scanner.tokens);

        let mut parser = Parser::new(scanner.tokens);
        let expr = parser.parse_expr();
        println!("Expr: \n{:#?}\n\n", expr);

        println!("Eval: \n{:?}\n\n", expr.eval());
    }
}
