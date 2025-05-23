use environment::Environment;
use runner::{run_file, run_input};
use std_functions::add_functions;

mod ast;
mod environment;
mod error;
mod eval;
mod parser;
mod runner;
mod scanner;
mod std_functions;
#[cfg(test)]
mod tests;

fn main() {
    let mut env = Environment::new();
    add_functions(&mut env);

    std::env::args().next();
    let args: Vec<_> = std::env::args().filter(|f| f.starts_with("--")).collect();
    let fp = std::env::args().filter(|f| !f.starts_with("--")).nth(1);

    let dbg = args.contains(&"--dbg".to_owned());
    if let Some(fp) = fp {
        run_file(&fp, &mut env, dbg);
    } else {
        loop {
            run_input(&mut env, dbg);
        }
    }
}
