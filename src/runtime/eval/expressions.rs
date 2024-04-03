use crate::frontend::ast::*;
use crate::runtime::environment::*;
use crate::runtime::interpreter::*;
use crate::runtime::values::*;

fn eval_numeric_binary_expr(lhs: NumberVal, rhs: NumberVal, operator: &str) -> RuntimeVal {
    let result = match operator {
        "+" => lhs.value + rhs.value,
        "-" => lhs.value - rhs.value,
        "*" => lhs.value * rhs.value,
        "/" => lhs.value / rhs.value, // TODO: Division by zero checks
        "%" => lhs.value % rhs.value,
        _ => panic!("Unexpected operator"),
    };

    MK_NUMBER(result)
}

pub fn eval_binary_expr(binop: BinaryExpr, env: &mut Environment) -> RuntimeVal {
    let lhs = match evaluate(Stmt::Expr(*binop.left), env) {
        RuntimeVal::Number(n) => n,
        _ => return MK_NULL(), // Return null if lhs is not a number
    };

    let rhs = match evaluate(Stmt::Expr(*binop.right), env) {
        RuntimeVal::Number(n) => n,
        _ => return MK_NULL(), // Return null if rhs is not a number
    };

    // Only currently support numeric operations
    match (&lhs.value_type, &rhs.value_type) {
        (ValueType::Number, ValueType::Number) => {
            eval_numeric_binary_expr(lhs, rhs, &binop.operator)
        }
        _ => MK_NULL(), // One or both are NULL
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
        },
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