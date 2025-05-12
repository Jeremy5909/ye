use std::env;

use inp_handling::{run_file, run_input};
use parser::{Parser, eval::Environment};

mod inp_handling;
mod parser;
mod scanner;
mod tests;
mod token;

fn main() {
    let mut env = Environment::new();
    if let Some(file_name) = env::args().nth(1) {
        run_file(file_name.as_str(), &mut env);
    } else {
        loop {
            run_input(&mut env, true);
        }
    }
}
