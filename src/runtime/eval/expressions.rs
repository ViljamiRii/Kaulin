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
                panic!("Division by zero error");
            } else {
                MK_NUMBER(lhs / rhs)
            }
        }
        BinaryOperator::Modulus => {
            if rhs == 0.0 {
                panic!("Division by zero error");
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
        _ => panic!("Unexpected operator"),
    }
}

pub fn eval_binary_expr(binop: BinaryExpr, env: &mut Environment) -> RuntimeVal {
    let lhs = evaluate(Stmt::Expr(*binop.left), env);
    let rhs = evaluate(Stmt::Expr(*binop.right), env);

    match (&lhs, &rhs) {
        (RuntimeVal::Number(n1), RuntimeVal::Number(n2)) => {
            let lhs = *n1;
            let rhs = *n2;
            eval_numeric_binary_expr(lhs, rhs, &binop.operator)
        }
        (RuntimeVal::String(s1), RuntimeVal::String(s2)) if
            binop.operator == BinaryOperator::Add
        => {
            RuntimeVal::String(s1.clone() + s2)
        }
        (RuntimeVal::Bool(b1), RuntimeVal::Bool(b2)) if
            binop.operator == BinaryOperator::And ||
            binop.operator == BinaryOperator::Or
        => {
            match binop.operator {
                BinaryOperator::And => RuntimeVal::Bool(*b1 && *b2),
                BinaryOperator::Or => RuntimeVal::Bool(*b1 || *b2),
                _ => panic!("Unexpected operator"),
            }
        }
        _ => panic!("Unsupported operand types for binary operation"),
    }
}

pub fn eval_identifier(ident: Identifier, env: &mut Environment) -> RuntimeVal {
    let val = env.lookup_var(&ident.symbol);
    val
}

pub fn eval_assignment(assignment_expr: AssignmentExpr, env: &mut Environment) -> RuntimeVal {
    match *assignment_expr.assignee {
        Expr::Identifier(ref ident) => {
            let value = evaluate(Stmt::Expr(*assignment_expr.value.clone()), env);
            env.assign_var(ident.symbol.clone(), value.clone());
            value
        }
        _ => panic!("Invalid LHS inside assignment expr {:?}", assignment_expr.assignee),
    }
}

pub fn eval_object_expr(obj: ObjectLiteral, env: &mut Environment) -> RuntimeVal {
    let mut properties = Vec::new();

    for property in obj.properties {
        let runtime_val = match property.value {
            Some(value) => evaluate(Stmt::Expr(*value), env),
            None => env.lookup_var(&property.key),
        };

        properties.push((property.key, runtime_val));
    }

    MK_OBJECT(properties)
}

pub fn eval_array_expr(array_literal: ArrayLiteral, env: &mut Environment) -> RuntimeVal {
    let mut runtime_vals = Vec::new();
    for expr in array_literal.elements {
        runtime_vals.push(evaluate(Stmt::Expr(*expr), env));
    }
    MK_ARRAY(runtime_vals)
}

pub fn eval_call_expr(expr: CallExpr, env: &mut Environment) -> RuntimeVal {
    let args: Vec<RuntimeVal> = expr.args
        .iter()
        .map(|arg| evaluate(Stmt::Expr((*arg).clone()), env))
        .collect();
    let fn_val = evaluate(Stmt::Expr(*expr.caller), env);

    match fn_val {
        RuntimeVal::NativeFunction(native_fn) => {
            let result = native_fn.get_fn()(args, env.variables.clone());
            result
        }
        RuntimeVal::Function(func) => {
            let mut scope = Environment::new(
                Some(Box::new(RefCell::borrow(&*func.declaration_env).clone()))
            );

            // Create the variables for the parameters list
            for (i, varname) in func.parameters.iter().enumerate() {
                // TODO Check the bounds here.
                // verify arity of function
                scope.declare_var(varname.clone(), args[i].clone(), false);
            }

            let mut result = MK_NULL();
            // Evaluate the function body line by line
            for stmt in &func.body {
                result = evaluate(stmt.clone(), &mut scope);
            }

            result
        }
        _ => panic!("Cannot call value that is not a function: {:?}", fn_val),
    }
}

pub fn eval_member_expr(expr: MemberExpr, env: &mut Environment) -> RuntimeVal {
    let object = evaluate(Stmt::Expr(*expr.object), env);
    let property = match *expr.property {
        Expr::Identifier(ident) => ident.symbol,
        _ => panic!("Property must be an identifier"),
    };

    match object {
        RuntimeVal::Object(obj) => {
            obj.iter()
                .find_map(|(key, val)| {
                    if key == &property { Some(val.clone()) } else { None }
                })
                .unwrap_or_else(|| panic!("Property '{}' does not exist on object", property))
        }
        _ => panic!("Only objects have properties"),
    }
}

pub fn eval_unary_expr(unary_expr: UnaryExpr, env: &mut Environment) -> RuntimeVal {
    let operand = evaluate(Stmt::Expr(*unary_expr.operand), env);
    match unary_expr.operator.as_str() {
        "-" => {
            if let RuntimeVal::Number(n) = operand {
                MK_NUMBER(-n)
            } else {
                panic!("Operand must be a number for unary '-' operator");
            }
        }
        "!" => {
            if let RuntimeVal::Bool(b) = operand {
                MK_BOOL(!b)
            } else {
                panic!("Operand must be a boolean for unary '!' operator");
            }
        }
        _ => panic!("Unexpected unary operator"),
    }
}

pub fn eval_if_else_expr(if_else_expr: IfElseExpr, env: &mut Environment) -> RuntimeVal {
    let condition = evaluate(Stmt::Expr(*if_else_expr.condition), env);
    match condition {
        RuntimeVal::Bool(b) => {
            if b {
                let mut result = MK_NULL();
                for stmt in if_else_expr.if_branch.statements {
                    result = evaluate(stmt, env);
                }
                result
            } else {
                match if_else_expr.else_branch {
                    Some(else_branch) => {
                        let mut result = MK_NULL();
                        for stmt in else_branch.statements {
                            result = evaluate(stmt, env);
                        }
                        result
                    }
                    None => MK_NULL(),
                }
            }
        }
        _ => panic!("Condition in if-else expression must evaluate to a boolean"),
    }
}