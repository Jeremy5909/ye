use std::{
    fs,
    io::{Write, stdin, stdout},
};

use crate::{
    ast::Value, environment::Environment, error::ParsingError, parser::Parser, scanner::Scanner,
};

pub fn run_file(file_name: &str, env: &mut Environment, dbg: bool) {
    let file = fs::read_to_string(file_name).expect("File not found");
    run(file, env, dbg).unwrap();
}

pub fn run_input(env: &mut Environment, dbg: bool) {
    let mut inp = String::new();
    let mut brace_depth = 0;
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        // i gotta fix ts
        brace_depth += line.matches(['[', '{', '(']).count();
        brace_depth -= line.matches([']', '}', ')']).count();
        inp.push_str(&line);
        if brace_depth == 0 {
            break;
        }
    }

    match run(inp, env, dbg) {
        Ok(Some(val)) => println!("{val}"),
        Err(err) => eprintln!("{err:?}"),
        _ => (),
    }
}

pub fn run(inp: String, env: &mut Environment, dbg: bool) -> Result<Option<Value>, ParsingError> {
    let inp = inp.trim();
    let mut scanner = Scanner::from(inp);
    scanner.scan_tokens();
    if dbg {
        println!("{:?}", scanner.tokens)
    }
    let mut parser = Parser::new(scanner.tokens);
    let stmts = parser.parse_all()?;
    for stmt in stmts {
        if dbg {
            println!("{stmt:?}")
        }
        stmt.eval(env)?;
    }
    Ok(None)
}
