use std::{
    env, fs,
    io::{Write, stdin, stdout},
};

use parser::{Parser, eval::Enviroment};
use scanner::Scanner;

mod parser;
mod scanner;
mod tests;
mod token;

fn read_file(file_name: &str, env: &mut Enviroment) {
    let file = fs::read_to_string(file_name).expect("File not found");
    let lines: Vec<_> = file.lines().collect();
    for line in lines.iter() {
        read_line(line.to_string(), env, false)
    }
}

fn read_input(env: &mut Enviroment, dbg: bool) {
    let mut inp = String::new();
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut inp).unwrap();
    read_line(inp, env, dbg);
}

fn read_line(line: String, env: &mut Enviroment, dbg: bool) {
    let mut scanner = Scanner::from(line.trim());
    scanner.scan_tokens();
    println!("Tokens:\n{:?}\n\n", scanner.tokens);

    let mut parser = Parser::new(scanner.tokens);
    let expr = parser.parse_expr();
    if dbg {
        println!("Expr: \n{:#?}\n\n", expr);
        println!("Eval: \n{:?}\n\n", expr.eval(env));
    }
}

fn main() {
    let mut env = Enviroment::new();
    if let Some(file_name) = env::args().nth(1) {
        read_file(file_name.as_str(), &mut env);
    } else {
        loop {
            read_input(&mut env, true);
        }
    }
}
