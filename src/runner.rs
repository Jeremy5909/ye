use std::{
    fs,
    io::{Write, stdin, stdout},
};

use crate::{environment::Environment, parser::Parser, scanner::Scanner};

pub fn run_file(file_name: &str, env: &mut Environment) {
    let file = fs::read_to_string(file_name).expect("File not found");
    run(file, env, false);
}

pub fn run_input(env: &mut Environment) {
    let mut inp = String::new();
    let mut brace_depth = 0;
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        brace_depth += line.matches(['[', '{', '(']).count();
        brace_depth -= line.matches([']', '}', ')']).count();
        inp.push_str(&line);
        if brace_depth == 0 && !inp.trim().is_empty() {
            break;
        }
    }
    run(inp, env, true)
}

pub fn run(inp: String, env: &mut Environment, dbg: bool) {
    let inp = inp.trim();
    let mut scanner = Scanner::from(inp);
    scanner.scan_tokens();
    if dbg {
        println!("{:?}", scanner.tokens)
    }
    let mut parser = Parser::new(scanner.tokens);
    let stmts = parser.parse_all().unwrap();
    for stmt in stmts {
        if dbg {
            println!("{stmt:?}")
        }
        if let Some(val) = stmt.eval(env).unwrap() {
            println!("Value: {val:?}")
        }
    }
}
