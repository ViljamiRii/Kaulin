use crate::runtime::values::{RuntimeVal, ValueType, NullVal, NumberVal};
use crate::frontend::ast::{Stmt, NumericLiteral, Identifier, BinaryExpr, Program, Expr};
use crate::runtime::environment::Environment;

fn eval_program(program: Program, env: &mut Environment) -> RuntimeVal {
    let mut last_evaluated = RuntimeVal::Null(NullVal::mk_null());
    for statement in program.body {
        last_evaluated = evaluate(statement, env);
    }
    last_evaluated
}

fn eval_numeric_binary_expr(lhs: NumberVal, rhs: NumberVal, operator: &str) -> RuntimeVal {
    let result = match operator {
        "+" => lhs.value + rhs.value,
        "-" => lhs.value - rhs.value,
        "*" => lhs.value * rhs.value,
        "/" => lhs.value / rhs.value, // TODO: Division by zero checks
        "%" => lhs.value % rhs.value,
        _ => panic!("Unexpected operator"),
    };

    RuntimeVal::Number(NumberVal::mk_number(result))
}

fn eval_binary_expr(binop: BinaryExpr, env: &mut Environment) -> RuntimeVal {
    let lhs = match evaluate(Stmt::Expr(*binop.left), env) {
        RuntimeVal::Number(n) => n,
        _ => panic!("Expected NumberVal"),
    };

    let rhs = match evaluate(Stmt::Expr(*binop.right), env) {
        RuntimeVal::Number(n) => n,
        _ => panic!("Expected NumberVal"),
    };

    // Only currently support numeric operations
    let lhs_clone = lhs.clone();
    let rhs_clone = rhs.clone();
    match (lhs_clone.value_type, rhs_clone.value_type) {
        (ValueType::Number, ValueType::Number) => {
            eval_numeric_binary_expr(lhs, rhs, &binop.operator)
        }
        _ => RuntimeVal::Null(NullVal::mk_null()), // One or both are NULL
    }
}

fn eval_identifier(ident: Identifier, env: &mut Environment) -> RuntimeVal {
    let val = env.lookup_var(&ident.symbol);
    val
}

pub fn evaluate(ast_node: Stmt, env: &mut Environment) -> RuntimeVal {
    match ast_node {
        Stmt::Expr(expr) => match expr {
            Expr::NumericLiteral(numeric_literal) => {
                RuntimeVal::Number(NumberVal::mk_number(numeric_literal.value))
            }
            Expr::Identifier(identifier) => eval_identifier(identifier, env),
            Expr::BinaryExpr(binary_expr) => eval_binary_expr(binary_expr, env),
            // Add other Expr variants here as needed
        },
        Stmt::NumericLiteral(numeric_literal) => {
            RuntimeVal::Number(NumberVal::mk_number(numeric_literal.value))
        }
        Stmt::Identifier(identifier) => eval_identifier(identifier, env),
        Stmt::BinaryExpr(binary_expr) => eval_binary_expr(binary_expr, env),
        Stmt::Program(program) => eval_program(program, env),
        _ => panic!("This AST Node has not yet been setup for interpretation: {:?}", ast_node),
    }
}