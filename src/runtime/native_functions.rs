use std::time::SystemTime;
use std::rc::Rc;
use std::f64;
use std::io::{ self, Write };
use rand::Rng;
use strfmt::strfmt;
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
            _ => panic!("round function takes an integer as the second argument"),
        }
    } else {
        0
    };
    let multiplier = (10f64).powi(ndigits);
    let mut result = (number * multiplier).round() / multiplier;
    if result == -0.0 {
        result = 0.0;
    }
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

pub fn random_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    if args.len() < 2 || args.len() > 3 {
        panic!("random function takes two or three arguments");
    }
    let min = match args.get(0) {
        Some(RuntimeVal::Number(n)) => *n,
        _ => panic!("random function takes a number as the first argument"),
    };
    let max = match args.get(1) {
        Some(RuntimeVal::Number(n)) => *n,
        _ => panic!("random function takes a number as the second argument"),
    };
    let mut rng = rand::thread_rng();
    let result = if args.len() == 3 {
        match args.get(2) {
            Some(RuntimeVal::String(s)) if s == "liukuluku" => rng.gen_range(min..max),
            Some(RuntimeVal::String(s)) if s == "kokonaisluku" =>
                rng.gen_range(min.floor() as i64..max.ceil() as i64) as f64,
            _ =>
                panic!(
                    "random function's third argument must be either 'kokonaisluku' or 'liukuluku'"
                ),
        }
    } else {
        rng.gen_range(min.floor() as i64..max.ceil() as i64) as f64
    };
    MK_NUMBER(result)
}

pub fn print_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let format_args = args
        .iter()
        .map(|arg| match arg {
            RuntimeVal::Number(n) => n.to_string(),
            RuntimeVal::String(s) => s.clone(),
            RuntimeVal::Object(o) => format!("{:?}", o),
            RuntimeVal::Array(a) => format!("{:?}", a),
            RuntimeVal::Number(i) => i.to_string(),
            RuntimeVal::Bool(b) => b.to_string(),
            RuntimeVal::Null => "null".to_string(),
            RuntimeVal::Function(f) => format!("{:?}", f),
            _ => panic!("print function takes only numbers, strings, objects, arrays, integers, floats, booleans, nulls, or functions as arguments"),
        })
        .collect::<Vec<String>>();
    let output = format_args.join(" ");
    println!("{}", output);
    MK_NULL()
}

pub fn max_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let numbers = match &args[0] {
        RuntimeVal::Array(arr) => arr,
        _ => panic!("max function expects an array of numbers as argument"),
    };

    if numbers.is_empty() {
        panic!("max function expects at least one argument");
    }

    let mut max_val = match &numbers[0] {
        RuntimeVal::Number(num) => num,
        _ => panic!("max function expects only numbers in the array"),
    };

    for val in &numbers[1..] {
        if let RuntimeVal::Number(num) = val {
            if num > max_val {
                max_val = num;
            }
        } else {
            panic!("max function expects only numbers in the array");
        }
    }

    RuntimeVal::Number(*max_val)
}

pub fn min_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let numbers = match &args[0] {
        RuntimeVal::Array(arr) => arr,
        _ => panic!("min function expects an array of numbers as argument"),
    };

    if numbers.is_empty() {
        panic!("min function expects at least one argument");
    }

    let mut min_val = match &numbers[0] {
        RuntimeVal::Number(num) => num,
        _ => panic!("min function expects only numbers in the array"),
    };

    for val in &numbers[1..] {
        if let RuntimeVal::Number(num) = val {
            if num < min_val {
                min_val = num;
            }
        } else {
            panic!("min function expects only numbers in the array");
        }
    }

    RuntimeVal::Number(*min_val)
}

pub fn length_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let array = match &args[0] {
        RuntimeVal::Array(arr) => arr,
        _ => panic!("length function expects an array as argument"),
    };

    RuntimeVal::Number(array.len() as f64)
}

pub fn sort_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let mut array = match &args[0] {
        RuntimeVal::Array(arr) => arr.clone(),
        _ => panic!("sort function expects an array as argument"),
    };

    array.sort_by(|a, b| match (a, b) {
        (RuntimeVal::Number(num_a), RuntimeVal::Number(num_b)) => num_a.partial_cmp(num_b).unwrap(),
        (RuntimeVal::String(str_a), RuntimeVal::String(str_b)) => str_a.cmp(str_b),
        _ => panic!("sort function expects an array of numbers or strings"),
    });

    RuntimeVal::Array(array)
}

pub fn reverse_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let mut array = match &args[0] {
        RuntimeVal::Array(arr) => arr.clone(),
        _ => panic!("reverse function expects an array as argument"),
    };

    array.reverse();

    RuntimeVal::Array(array)
}
