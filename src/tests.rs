#[allow(unused_imports)]
use crate::parser::Value;
use crate::{inp_handling::run_line, parser::eval::Environment};

#[allow(dead_code)]
fn test(commands: &str, env: &mut Environment) {
    commands.lines().for_each(|line| {
        run_line(line.to_string(), env, true);
    });
}

#[test]
fn assign() {
    let mut env = Environment::new();
    test(
        "let x=5 
        x",
        &mut env,
    );
    assert_eq!(*env.get("x").unwrap(), Value::Number(5.0))
}

#[test]
fn assign_to_var() {
    let mut env = Environment::new();
    test(
        "let x=5
        let y=x",
        &mut env,
    );
    assert_eq!(*env.get("x").unwrap(), Value::Number(5.0));
    assert_eq!(*env.get("y").unwrap(), Value::Number(5.0));
}

#[test]
fn simple_addition() {
    let mut env = Environment::new();
    test(
        "let x=201
        let y=30
        let z=5
        x+y
        y+z
        x+z
        let w=x+y+z",
        &mut env,
    );
    assert_eq!(*env.get("w").unwrap(), Value::Number(201.0 + 30.0 + 5.0))
}

#[test]
fn string_addition() {
    let mut env = Environment::new();
    test(
        "let s1 = \"hello\"
        let s2 = \"world!\"
        let space = \" \"
        let result = s1+space+s2
        ",
        &mut env,
    );
    assert_eq!(
        *env.get("result").unwrap(),
        Value::Str("hello world!".to_string())
    )
}
