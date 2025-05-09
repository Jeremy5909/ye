use std::env;

use inp_handling::{read_file, read_input};
use parser::{Parser, eval::Enviroment};

mod inp_handling;
mod parser;
mod scanner;
mod token;

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
