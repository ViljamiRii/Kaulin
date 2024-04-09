use crate::frontend::ast::*;
use crate::runtime::environment::*;
use crate::runtime::interpreter::*;
use crate::runtime::values::*;
use crate::runtime::eval::statements::*;
use std::cell::RefCell;

fn eval_numeric_binary_expr(lhs: f64, rhs: f64, operator: &BinaryOperator) -> RuntimeVal {
    match operator {
        BinaryOperator::Add => MK_NUMBER(lhs + rhs),
        BinaryOperator::Subtract => MK_NUMBER(lhs - rhs),
        BinaryOperator::Multiply => MK_NUMBER(lhs * rhs),
        BinaryOperator::Divide => {
            if rhs == 0.0 {
                panic!("Jako nollavirheellä");
            } else {
                MK_NUMBER(lhs / rhs)
            }
        }
        BinaryOperator::Modulus => {
            if rhs == 0.0 {
                panic!("Jako nollavirheellä");
            } else {
                MK_NUMBER(lhs % rhs)
            }
        }
        BinaryOperator::And => {
            if lhs != 0.0 && rhs != 0.0 { MK_BOOL(true) } else { MK_BOOL(false) }
        }
        BinaryOperator::Or => {
            if lhs != 0.0 || rhs != 0.0 { MK_BOOL(true) } else { MK_BOOL(false) }
        }
        BinaryOperator::Exponent => MK_NUMBER(lhs.powf(rhs)),
        BinaryOperator::Equal => MK_BOOL(lhs == rhs),
        BinaryOperator::NotEqual => MK_BOOL(lhs != rhs),
        BinaryOperator::LessThan => MK_BOOL(lhs < rhs),
        BinaryOperator::GreaterThan => MK_BOOL(lhs > rhs),
        BinaryOperator::LessThanOrEqual => MK_BOOL(lhs <= rhs),
        BinaryOperator::GreaterThanOrEqual => MK_BOOL(lhs >= rhs),
        _ => panic!("Odottamaton operaattori"),
    }
}

pub fn eval_binary_expr(binop: &BinaryExpr, env: &mut Environment) -> RuntimeVal {
    let lhs = eval_expr(&*binop.left, env);
    let rhs = eval_expr(&*binop.right, env);

    match binop.operator {
        BinaryOperator::Add => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Number(n1 + n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Integer(i1 + i2),
            (RuntimeVal::String(s1), RuntimeVal::String(s2)) => RuntimeVal::String(s1.clone() + s2),
            _ => panic!("Tukematon operandityyppi yhteenlaskuun"),
        },
        BinaryOperator::Subtract => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Number(n1 - n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Integer(i1 - i2),
            _ => panic!("Tukematon operandityyppi vähennyslaskuun"),
        },
        BinaryOperator::Multiply => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Number(n1 * n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Integer(i1 * i2),
            _ => panic!("Tukematon operandityyppi kertolaskuun"),
        },
        BinaryOperator::Divide => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Number(n1 / n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Integer(i1 / i2),
            _ => panic!("Tukematon operandityyppi jakolaskuun"),
        },
        BinaryOperator::Exponent => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Number(n1.powf(*n2)),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Integer(i1.pow(*i2 as u32)),
            _ => panic!("Tukematon operandityyppi potenssiin"),
        },
        BinaryOperator::Modulus => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Number(n1 % n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Integer(i1 % i2),
            _ => panic!("Tukematon operandityyppi jakojäännökseen"),
        },
        BinaryOperator::AddEqual => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Number(*n1 + *n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Integer(*i1 + *i2),
            _ => panic!("Tukematon operandityyppi yhteenlaskuun ja sijoitukseen"),
        },
        BinaryOperator::SubtractEqual => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Number(*n1 - *n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Integer(*i1 - *i2),
            _ => panic!("Tukematon operandityyppi vähennyslaskuun ja sijoitukseen"),
        },
        BinaryOperator::Equal => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Bool(n1 == n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Bool(i1 == i2),
            (RuntimeVal::String(s1), RuntimeVal::String(s2)) => RuntimeVal::Bool(s1 == s2),
            _ => panic!("Tukematon operandityyppi yhtäsuuruuden tarkistukseen"),
        },
        BinaryOperator::NotEqual => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Bool(n1 != n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Bool(i1 != i2),
            (RuntimeVal::String(s1), RuntimeVal::String(s2)) => RuntimeVal::Bool(s1 != s2),
            _ => panic!("Tukematon operandityyppi erisuuruuden tarkistukseen"),
        },
        BinaryOperator::LessThan => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Bool(n1 < n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Bool(i1 < i2),
            _ => panic!("Tukematon operandityyppi pienempi kuin -tarkistukseen"),
        },
        BinaryOperator::GreaterThan => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Bool(n1 > n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Bool(i1 > i2),
            _ => panic!("Tukematon operandityyppi suurempi kuin -tarkistukseen"),
        },
        BinaryOperator::LessThanOrEqual => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Bool(n1 <= n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Bool(i1 <= i2),
            _ => panic!("Tukematon operandityyppi pienempi tai yhtä suuri kuin -tarkistukseen"),
        },
        BinaryOperator::GreaterThanOrEqual => match (&lhs, &rhs) {
            (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => RuntimeVal::Bool(n1 >= n2),
            (RuntimeVal::Integer(i1), RuntimeVal::Integer(i2)) => RuntimeVal::Bool(i1 >= i2),
            _ => panic!("Tukematon operandityyppi suurempi tai yhtä suuri kuin -tarkistukseen"),
        },
        _ => panic!("Tukematon operaattori"),
    }
}

pub fn eval_identifier(ident: &Identifier, env: &mut Environment) -> RuntimeVal {
    let val = env.lookup_var(&ident.symbol);
    val
}

pub fn eval_assignment(assignment_expr: &AssignmentExpr, env: &mut Environment) -> RuntimeVal {
    match &*assignment_expr.assignee {
        Expr::Identifier(ident) => {
            let value = eval_expr(&assignment_expr.value, env);
            env.assign_var(&ident.symbol, &value);
            value
        }
        _ => panic!("Virheellinen vasen puoli lausekkeen sisällä {:?}", assignment_expr.assignee),
    }
}

pub fn eval_object_expr(obj: &ObjectLiteral, env: &mut Environment) -> RuntimeVal {
    let mut properties = Vec::new();

    for property in &obj.properties {
        let runtime_val = match &property.value {
            Some(value) => eval_expr(value, env),
            None => env.lookup_var(&property.key),
        };

        properties.push((property.key.clone(), runtime_val));
    }

    MK_OBJECT(properties)
}

pub fn eval_array_expr(array_literal: &ArrayLiteral, env: &mut Environment) -> RuntimeVal {
    let runtime_vals: Vec<RuntimeVal> = array_literal.elements.iter().map(|expr| eval_expr(expr, env)).collect();
    MK_ARRAY(runtime_vals)
}

pub fn eval_call_expr(expr: &CallExpr, env: &mut Environment) -> RuntimeVal {
    let args: Vec<RuntimeVal> = expr.args.iter().map(|arg| eval_expr(arg, env)).collect();
    let fn_val = eval_expr(&expr.caller, env);


    match fn_val {
        RuntimeVal::NativeFunction(native_fn) => {
            let result = native_fn.get_fn()(args, env.variables.clone());
            result
        }
        RuntimeVal::Function(func) => {
            let mut scope = Environment::new(
                Some(Box::new(RefCell::borrow(&*func.declaration_env).clone()))
            );

            // Check the bounds here.
            // Verify arity of function
            if func.parameters.len() != args.len() {
                panic!("Väärä argumenttien määrä. Odotettu {}, saatu {}", func.parameters.len(), args.len());
            }

            // Create the variables for the parameters list
            for (i, varname) in func.parameters.iter().enumerate() {
                scope.declare_var(varname.clone(), args[i].clone(), false);
            }

            let mut result = MK_NULL();
            // Evaluate the function body line by line
            for stmt in &func.body {
                result = evaluate(&stmt.clone(), &mut scope);
            }

            result
        }
        _ => panic!("Ei voida kutsua arvoa, joka ei ole funktio: {:?}", fn_val),
    }
}

pub fn eval_member_expr(expr: &MemberExpr, env: &mut Environment) -> RuntimeVal {
    let object = eval_expr(&expr.object, env);
    let property = match &*expr.property {
        Expr::Identifier(ident) => ident.symbol.clone(),
        _ => panic!("Omaisuuden on oltava tunniste"),
    };

    match object {
        RuntimeVal::Object(obj) => {
            obj.iter()
                .find_map(|(key, val)| {
                    if key == &property { Some(val.clone()) } else { None }
                })
                .unwrap_or_else(|| panic!("Oliossa ei ole ominaisuutta {}", property))
        }
        _ => panic!("Vain olioilla on ominaisuuksia"),
    }
}

pub fn eval_unary_expr(unary_expr: &UnaryExpr, env: &mut Environment) -> RuntimeVal {
    let operand = eval_expr(&unary_expr.operand, env);
    match unary_expr.operator.as_str() {
        "-" => {
            if let RuntimeVal::Number(n) = operand {
                MK_NUMBER(-n)
            } else {
                panic!("Operandin on oltava unaarisen '-'-operaattorin numero");
            }
        }
        "!" => {
            if let RuntimeVal::Bool(b) = operand {
                MK_BOOL(!b)
            } else {
                panic!("Operandin on oltava unaarisen '!'-operaattorin totuusarvo");
            }
        }
        _ => panic!("Odottamaton unaarinen operaattori"),
    }
}

pub fn eval_if_else_expr(if_else_expr: &IfElseExpr, env: &mut Environment) -> RuntimeVal {
    let condition = eval_expr(&if_else_expr.condition, env);
    match condition {
        RuntimeVal::Bool(b) => {
            if b {
                let mut result = MK_NULL();
                for stmt in &if_else_expr.if_branch.statements {
                    result = evaluate(&stmt.clone(), env);
                }
                result
            } else {
                match &if_else_expr.else_branch {
                    Some(else_branch) => {
                        let mut result = MK_NULL();
                        for stmt in &else_branch.statements {
                            result = evaluate(&stmt.clone(), env);
                        }
                        result
                    }
                    None => MK_NULL(),
                }
            }
        }
        _ => panic!("jos-muuten-lausekkeen ehto on arvioitava totuusarvoksi"),
    }
}

pub fn eval_string_literal(string_literal: &StringLiteral, env: &mut Environment, args: &[RuntimeVal]) -> RuntimeVal {
    let mut output = string_literal.value.clone();
    for arg in args {
        output = output.replace("%{}", &format!("{}", arg)); // Replace first occurrence of "%{}" with the argument
    }
    MK_STRING(output)
}