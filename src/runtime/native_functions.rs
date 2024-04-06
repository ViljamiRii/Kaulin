use std::time::SystemTime;
use std::rc::Rc;
use std::f64;
use std::io::{self, Write};
use crate::runtime::values::*;

pub fn time_function(_args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let now = SystemTime::now();
    MK_NUMBER(now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64())
}

pub fn abs_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    match args.get(0) {
        Some(RuntimeVal::Number(n)) => MK_NUMBER(n.abs()),
        _ => panic!("abs() expects a number as an argument"),
    }
}

pub fn round_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    if args.len() > 2 {
        panic!("round function takes at most two arguments");
    }
    let number = match args.get(0) {
        Some(RuntimeVal::Number(n)) => *n,
        _ => panic!("round function takes a number as the first argument"),
    };
    let ndigits = if args.len() == 2 {
        match args.get(1) {
            Some(RuntimeVal::Number(n)) => *n as i32,
            _ => panic!("pyöristä function takes an integer as the second argument"),
        }
    } else {
        0
    };
    let multiplier = (10f64).powi(-ndigits);
    let result = (number / multiplier).round() * multiplier;
    MK_NUMBER(result)
}

pub fn sqrt_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    if args.len() != 1 {
        panic!("neliöjuuri function takes exactly one argument");
    }
    let number = match args.get(0) {
        Some(RuntimeVal::Number(n)) => *n,
        _ => panic!("neliöjuuri function takes a number as an argument"),
    };
    if number < 0.0 {
        panic!("neliöjuuri function cannot take a negative number as an argument");
    }
    let result = number.sqrt();
    MK_NUMBER(result)
}

pub fn input_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    if !args.is_empty() {
        panic!("input function does not take any arguments");
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    MK_STRING(input)
}
