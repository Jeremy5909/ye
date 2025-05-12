use inp_handling::{run_file, run_input};
use parser::eval::Environment;

mod inp_handling;
mod parser;
mod scanner;
mod tests;
mod token;

fn main() {
    let mut env = Environment::new();
    std::env::args().next();
    let flags: Vec<_> = std::env::args().filter(|f| f.starts_with("--")).collect();
    let fp = std::env::args().filter(|f| !f.starts_with("--")).nth(1);

    let dbg = flags.contains(&"--dbg".to_owned());
    if let Some(fp) = fp {
        run_file(&fp, &mut env, dbg);
    } else {
        loop {
            run_input(&mut env, dbg);
        }
    }
}
