use std::{
    fs,
    io::{Write, stdin, stdout},
};

use crate::{
    parser::{Parser, eval::Environment},
    scanner::Scanner,
};

pub fn run_file(file_name: &str, env: &mut Environment) {
    let file = fs::read_to_string(file_name).expect("File not found");
    let lines: Vec<_> = file.lines().collect();
    lines
        .iter()
        .for_each(|line| run_line(line.to_string(), env, false));
}

pub fn run_input(env: &mut Environment, dbg: bool) {
    let mut inp = String::new();
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut inp).unwrap();
    run_line(inp, env, dbg);
}

pub fn run_line(line: String, env: &mut Environment, dbg: bool) {
    // todo have this be in scanner or wtv instead
    if line.trim().is_empty() {
        return;
    }
    let mut scanner = Scanner::from(line.trim());
    scanner.scan_tokens();
    if dbg {
        println!("Tokens:\n{:?}\n\n", scanner.tokens);
    }

    let mut parser = Parser::new(scanner.tokens);
    let stmt = parser.parse().unwrap();
    if dbg {
        if let Some(val) = stmt.eval(env).unwrap() {
            println!("Value: {val:#?}\n");
        }
    }
}
