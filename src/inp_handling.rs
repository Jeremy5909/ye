use std::{
    fs,
    io::{Write, stdin, stdout},
};

use crate::{
    parser::{Parser, eval::Enviroment},
    scanner::Scanner,
};

pub fn read_file(file_name: &str, env: &mut Enviroment) {
    let file = fs::read_to_string(file_name).expect("File not found");
    let lines: Vec<_> = file.lines().collect();
    for line in lines.iter() {
        read_line(line.to_string(), env, false)
    }
}

pub fn read_input(env: &mut Enviroment, dbg: bool) {
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
