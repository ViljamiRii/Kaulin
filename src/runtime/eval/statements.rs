use crate::frontend::ast::*;
use crate::runtime::environment::*;
use crate::runtime::interpreter::*;
use crate::runtime::values::*;


use std::rc::Rc;
use std::cell::RefCell;

pub fn eval_program(program: &Program, env: &mut Environment) -> RuntimeVal {
    let mut last_evaluated = MK_NULL();
    for statement in &program.body {
        last_evaluated = evaluate(statement, env);
    }
    last_evaluated
}

pub fn eval_var_declaration(var_declaration: &VarDeclaration, env: &mut Environment) -> RuntimeVal {
    let VarDeclaration { constant, identifier, value } = var_declaration;
    let value = match value {
        Some(expr) => evaluate(&Stmt::Expr(expr.clone()), env),
        None => MK_NULL(),
    };

    env.declare_var(identifier.symbol.clone(), value, *constant)
}

pub fn eval_function_declaration(
    declaration: &FunctionDeclaration,
    env: &mut Environment
) -> RuntimeVal {
    let function = Function {
        parameters: declaration.parameters.clone(),
        declaration_env: Rc::new(RefCell::new(env.clone())),
        body: declaration.body.clone(),
    };

    let function_val = RuntimeVal::Function(function);

    env.declare_var(declaration.name.clone(), function_val, true)
}

pub fn eval_while_loop(while_loop: &WhileLoop, env: &mut Environment) -> RuntimeVal {
    let mut result = RuntimeVal::Null;
    while eval_expr(&while_loop.condition, env).is_truthy() {
        result = eval_block(&while_loop.body.clone(), env);
    }
    result
}

pub fn eval_for_loop(for_loop: &ForLoop, env: &mut Environment) -> RuntimeVal {
    let ForLoop { initializer, condition, increment, body } = for_loop;
    evaluate(initializer, env);
    while match eval_expr(condition, env) {
        RuntimeVal::Bool(b) => b,
        _ => false,
    } {
        eval_block(body, env);
        eval_expr(increment, env);
    }
    MK_NULL()
}

fn eval_block(block: &Block, env: &mut Environment) -> RuntimeVal {
    let mut result = RuntimeVal::Null;
    for stmt in &block.statements {
        result = evaluate(stmt, env);
    }
    result
}