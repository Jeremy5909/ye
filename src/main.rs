use std::io::{Write, stdin, stdout};

use parser::{Parser, eval::Enviroment};
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
        let mut env = Enviroment::new();
        env.set(String::from("x"), parser::Value::Number(420.0));
        println!("Eval: \n{:?}\n\n", expr.eval(&env));
    }
}
