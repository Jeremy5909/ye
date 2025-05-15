use crate::{ast::Value, environment::Environment, runner::run};

#[allow(unused_imports)]
#[allow(dead_code)]
fn test(commands: &str, env: &mut Environment) {
    run(commands.to_owned(), env, true).unwrap();
}

#[test]
fn assign() {
    let mut env = Environment::new();
    test(
        "
        let x=5 
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

#[test]
fn empty_func() {
    let mut env = Environment::new();
    test("let func = fn[] {}", &mut env)
}

#[test]
fn one_inp_func() {
    let mut env = Environment::new();
    test(
        "let square = fn[ x ] {
            x*x
        }
        let result = square:3",
        &mut env,
    );
    assert_eq!(*env.get("result").unwrap(), Value::Number(9.0));
}

#[test]
fn two_inp_func() {
    let mut env = Environment::new();
    test(
        "let add = fn[x, y] {
            x+y
        }
        let result = add:[4,7]",
        &mut env,
    );
    assert_eq!(*env.get("result").unwrap(), Value::Number(11.0));
}

#[test]
fn anon_func() {
    let mut env = Environment::new();
    test(
        "let s = fn[string]{string+\" \"+string}:\"hello\"",
        &mut env,
    );
    assert_eq!(*env.get("s").unwrap(), Value::Str("hello hello".to_owned()));
}

#[test]
fn num_comparison() {
    let mut env = Environment::new();
    test(
        "let a=3<5
        let b=5>3
        let c=3<=3",
        &mut env,
    );
    assert_eq!(*env.get("a").unwrap(), Value::Bool(true));
    assert_eq!(*env.get("b").unwrap(), Value::Bool(true));
    assert_eq!(*env.get("c").unwrap(), Value::Bool(true));
}

#[test]
fn or_comparison() {
    let mut env = Environment::new();
    test(
        "let a=false|false
        let b=false|true
        let c=true|false
        let d=true|true",
        &mut env,
    );
    assert_eq!(*env.get("a").unwrap(), Value::Bool(false));
    assert_eq!(*env.get("b").unwrap(), Value::Bool(true));
    assert_eq!(*env.get("c").unwrap(), Value::Bool(true));
    assert_eq!(*env.get("d").unwrap(), Value::Bool(true));
}

#[test]
fn and_comparison() {
    let mut env = Environment::new();
    test(
        "let a=false&false
        let b=false&true
        let c=true&false
        let d=true&true",
        &mut env,
    );
    assert_eq!(*env.get("a").unwrap(), Value::Bool(false));
    assert_eq!(*env.get("b").unwrap(), Value::Bool(false));
    assert_eq!(*env.get("c").unwrap(), Value::Bool(false));
    assert_eq!(*env.get("d").unwrap(), Value::Bool(true));
}

#[test]
fn assign_conditional() {
    let mut env = Environment::new();
    test("let x = if 1+7 == 4*2 {\"wow\"} else {\"bruh\"}", &mut env);
    assert_eq!(*env.get("x").unwrap(), Value::Str("wow".to_owned()));
}

#[test]
fn factorial() {
    let mut env = Environment::new();
    test(
        " let factorial = fn[number] {
          if number <= 0 {
            0
          } else {
            if number == 0 | number == 1 {
              1
            } else {
              number * factorial:number-1
            }
          }
        }
        let a = factorial:-2
        let b = factorial:-1
        let c = factorial:0
        let d = factorial:1
        let e = factorial:2
        let f = factorial:3 ",
        &mut env,
    );
    assert_eq!(*env.get("a").unwrap(), Value::Number(0.0));
    assert_eq!(*env.get("b").unwrap(), Value::Number(0.0));
    assert_eq!(*env.get("c").unwrap(), Value::Number(0.0));
    assert_eq!(*env.get("d").unwrap(), Value::Number(1.0));
    assert_eq!(*env.get("e").unwrap(), Value::Number(2.0));
    assert_eq!(*env.get("f").unwrap(), Value::Number(6.0));
}

#[test]
fn list_comp() {
    let mut env = Environment::new();
    test("let res = [1,2,3,4,5,6]::x{if x != 3 {x*2}}", &mut env);
    assert_eq!(
        *env.get("res").unwrap(),
        Value::Array(vec![
            Value::Number(2.0),
            Value::Number(4.0),
            Value::Number(8.0),
            Value::Number(10.0),
            Value::Number(12.0),
        ])
    )
}
