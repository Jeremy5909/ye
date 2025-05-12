use std::{
    fs,
    io::{Write, stdin, stdout},
};

use crate::{
    parser::{Parser, eval::Environment},
    scanner::Scanner,
};

pub fn run_file(file_name: &str, env: &mut Environment, dbg: bool) {
    let file = fs::read_to_string(file_name).expect("File not found");
    run(file, env, dbg);
}

pub fn run_input(env: &mut Environment, dbg: bool) {
    let mut inp = String::new();
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut inp).unwrap();
    run(inp, env, dbg);
}

pub fn run(inp: String, env: &mut Environment, dbg: bool) {
    let inp = inp.trim();
    // todo have this be in scanner or wtv instead
    if inp.is_empty() {
        return;
    }
    let mut scanner = Scanner::from(inp);
    scanner.scan_tokens();
    if dbg {
        println!("Tokens:\n{:#?}\n\n", scanner.tokens);
    }
    let mut parser = Parser::new(scanner.tokens);
    let stmt = parser.parse().unwrap();
    if dbg {
        if let Some(val) = stmt.eval(env).unwrap() {
            println!("Value: {val:#?}\n");
        }
    }
}
