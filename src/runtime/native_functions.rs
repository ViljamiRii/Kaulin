use std::time::SystemTime;
use std::rc::Rc;
use std::f64;
use std::io::{ self, Write };
use std::convert::TryInto;
use rand::Rng;
use strfmt::strfmt;
use crate::runtime::values::*;

pub fn time_function(_args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let millis = duration_since_epoch.as_millis();
    MK_INTEGER(millis as i64)
}

pub fn millis_to_seconds_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    match args.get(0) {
        Some(RuntimeVal::Integer(n)) => {
            let seconds = *n / 1000;
            MK_INTEGER(seconds)
        },
        _ => panic!("sekunnit-funktio odottaa kokonaislukua (millisekuntti) argumenttina"),
    }
}

pub fn abs_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    match args.get(0) {
        Some(RuntimeVal::Number(n)) => MK_NUMBER(n.abs()),
        Some(RuntimeVal::Integer(i)) => MK_INTEGER(i.abs()),
        _ => panic!("itseisarvo-funktio odottaa numeroa argumenttina"),
    }
}

pub fn round_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    if args.len() > 2 {
        panic!("pyöristä-funktio ottaa enintään kaksi argumenttia");
    }
    let number = match args.get(0) {
        Some(RuntimeVal::Number(n)) => *n,
        Some(RuntimeVal::Integer(i)) => *i as f64,
        _ => panic!("pyöristä-funktio ottaa luvun ensimmäisenä argumenttina"),
    };
    let ndigits = if args.len() == 2 {
        match args.get(1) {
            Some(RuntimeVal::Number(n)) => *n as i32,
            _ => panic!("pyöristä-funktio ottaa kokonaisluvun toiseksi argumentiksi"),
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
        panic!("neliöjuuri-funktio ottaa täsmälleen yhden argumentin");
    }
    let number = match args.get(0) {
        Some(RuntimeVal::Number(n)) => *n,
        Some(RuntimeVal::Integer(i)) => *i as f64,
        _ => panic!("neliöjuuri-funktio ottaa luvun argumenttina"),
    };
    if number < 0.0 {
        panic!("neliöjuuri-funktio ei voi ottaa negatiivista lukua argumenttina");
    }
    let result = number.sqrt();
    MK_NUMBER(result)
}

pub fn input_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    if !args.is_empty() {
        panic!("syöttötoiminto ei ota argumentteja");
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Rivin lukeminen epäonnistui");
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
        panic!("satunnainen-funktio ottaa kaksi tai kolme argumenttia");
    }
    let min = match args.get(0) {
        Some(RuntimeVal::Number(n)) => *n,
        Some(RuntimeVal::Integer(i)) => *i as f64,
        _ => panic!("satunnainen-funktio ottaa luvun ensimmäiseksi argumentiksi"),
    };
    let max = match args.get(1) {
        Some(RuntimeVal::Number(n)) => *n,
        Some(RuntimeVal::Integer(i)) => *i as f64,
        _ => panic!("satunnainen-funktio ottaa luvun toiseksi argumentiksi"),
    };
    let mut rng = rand::thread_rng();
    let result = if args.len() == 3 {
        match args.get(2) {
            Some(RuntimeVal::String(s)) if s == "liukuluku" => rng.gen_range(min..max),
            Some(RuntimeVal::String(s)) if s == "kokonaisluku" =>
                rng.gen_range(min.floor() as i64..max.ceil() as i64) as f64,
            _ =>
                panic!(
                    "satunnainen-funktion kolmannen argumentin on oltava joko 'kluku' tai 'lluku'"
                ),
        }
    } else {
        rng.gen_range(min.floor() as i64..max.ceil() as i64) as f64
    };
    MK_NUMBER(result)
}

pub fn print_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let mut args_iter = args.iter();

    let format_args = args_iter
        .next()
        .map(|arg| match arg {
            RuntimeVal::String(s) => {
                let parts: Vec<&str> = s.split("%{}").collect();
                let mut result = parts[0].to_string();
                for (part, arg) in parts[1..].iter().zip(&args[1..]) {
                    result.push_str(&arg.to_string());
                    result.push_str(part);
                }
                result
            },
            _ => arg.to_string(), // Convert non-string arguments into strings
        })
        .unwrap_or_else(|| String::new());

    println!("{}", format_args);
    MK_NULL()
}

pub fn max_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let numbers = match &args[0] {
        RuntimeVal::Array(arr) => arr,
        _ => panic!("maksimi-funktio odottaa numero joukkoa argumenttina"),
    };

    if numbers.is_empty() {
        panic!("maksimi-funktio odottaa vähintään yhtä argumenttia");
    }

    let mut max_val = match &numbers[0] {
        RuntimeVal::Number(num) => *num,
        RuntimeVal::Integer(i) => *i as f64,
        _ => panic!("maksimi-funktio odottaa vain numeroita taulukossa"),
    };

    for val in &numbers[1..] {
        match val {
            RuntimeVal::Number(num) => {
                if *num > max_val {
                    max_val = *num;
                }
            },
            RuntimeVal::Integer(i) => {
                let val = *i as f64;
                if val > max_val {
                    max_val = val;
                }
            },
            _ => panic!("maksimi-funktio odottaa vain numeroita taulukossa"),
        }
    }

    RuntimeVal::Number(max_val)
}

pub fn min_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let numbers = match &args[0] {
        RuntimeVal::Array(arr) => arr,
        _ => panic!("minimi-funktio odottaa numero joukkoa argumenttina"),
    };

    if numbers.is_empty() {
        panic!("minimi-funktio odottaa vähintään yhden argumentin");
    }

    let mut min_val = match &numbers[0] {
        RuntimeVal::Number(num) => *num,
        RuntimeVal::Integer(i) => *i as f64,
        _ => panic!("minimi-funktio odottaa vain numeroita taulukossa"),
    };

    for val in &numbers[1..] {
        match val {
            RuntimeVal::Number(num) => {
                if *num < min_val {
                    min_val = *num;
                }
            },
            RuntimeVal::Integer(i) => {
                let val = *i as f64;
                if val < min_val {
                    min_val = val;
                }
            },
            _ => panic!("minimi-funktio odottaa vain numeroita taulukossa"),
        }
    }

    RuntimeVal::Number(min_val)
}

pub fn length_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let array = match &args[0] {
        RuntimeVal::Array(arr) => arr,
        _ => panic!("pituus-funktio odottaa taulukon argumenttina"),
    };

    RuntimeVal::Number(array.len() as f64)
}

pub fn sort_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let mut array = match &args[0] {
        RuntimeVal::Array(arr) => arr.clone(),
        _ => panic!("järjestä-funktio odottaa taulukon argumenttina"),
    };

    array.sort_by(|a, b| match (a, b) {
        (RuntimeVal::Number(num_a), RuntimeVal::Number(num_b)) => num_a.partial_cmp(num_b).unwrap(),
        (RuntimeVal::String(str_a), RuntimeVal::String(str_b)) => str_a.cmp(str_b),
        (RuntimeVal::Integer(int_a), RuntimeVal::Integer(int_b)) => int_a.cmp(int_b),
        _ => panic!("järjestä-funktio odottaa joukon numeroita tai merkkijonoja"),
    });

    RuntimeVal::Array(array)
}

pub fn reverse_function(args: Vec<RuntimeVal>, _: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    let mut array = match &args[0] {
        RuntimeVal::Array(arr) => arr.clone(),
        _ => panic!("käänteinen-funktio odottaa taulukkoa argumenttina"),
    };

    array.reverse();

    RuntimeVal::Array(array)
}

pub fn kluku_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    match args.get(0) {
        Some(RuntimeVal::String(s)) => {
            match s.parse::<f64>() {
                Ok(n) => MK_INTEGER(n.floor() as i64),
                Err(_) => panic!("kluku() expects a string that can be parsed into a number"),
            }
        },
        Some(RuntimeVal::Integer(i)) => MK_INTEGER(*i),
        Some(RuntimeVal::Number(n)) => MK_INTEGER(n.floor() as i64),
        _ => panic!("kluku-funktio odottaa merkkijonoa, joka voidaan jäsentää luvuksi"),
    }
}

pub fn lluku_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    match args.get(0) {
        Some(RuntimeVal::String(s)) => {
            match s.parse::<f64>() {
                Ok(n) => MK_NUMBER(n),
                Err(_) => panic!("lluku-funktio odottaa merkkijonoa, joka voidaan jäsentää luvuksi"),
            }
        },
        Some(RuntimeVal::Integer(i)) => MK_NUMBER(*i as f64),
        Some(RuntimeVal::Number(n)) => MK_NUMBER(*n),
        _ => panic!("lluku-funktio odottaa argumenttina merkkijonoa, klukua tai llukua"),
    }
}

pub fn mjono_function(args: Vec<RuntimeVal>, _scope: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    match args.get(0) {
        Some(val) => MK_STRING(val.to_string()),
        _ => panic!("mjono-funktio odottaa argumenttia"),
    }
}