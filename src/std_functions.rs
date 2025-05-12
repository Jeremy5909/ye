use std::io::{Write, stdin, stdout};

use crate::parser::{ast::Value, environment::Environment};

fn print(args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
        print!("{arg}");
    }
    Ok(Value::Bool(true))
}
fn println(args: Vec<Value>) -> Result<Value, String> {
    let res = print(args);
    println!();
    res
}
fn input(_: Vec<Value>) -> Result<Value, String> {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .map_err(|_| "Error getting input")?;
    Ok(Value::Str(buf.trim_end().to_owned()))
}
fn prompt(args: Vec<Value>) -> Result<Value, String> {
    print(args.clone())?;
    stdout().flush().map_err(|_| "Error flushing stdout")?;
    input(args)
}
fn promptln(args: Vec<Value>) -> Result<Value, String> {
    println(args.clone())?;
    stdout().flush().map_err(|_| "Error flushing stdout")?;
    input(args)
}

pub fn add_functions(env: &mut Environment) {
    env.set("print", Value::NativeFunction(print));
    env.set("println", Value::NativeFunction(println));
    env.set("input", Value::NativeFunction(input));
    env.set("prompt", Value::NativeFunction(prompt));
    env.set("promptln", Value::NativeFunction(promptln));
}
