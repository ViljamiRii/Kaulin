use crate::frontend::ast::*;
use crate::runtime::environment::*;
use crate::runtime::interpreter::*;
use crate::runtime::values::*;

pub fn eval_program(program: Program, env: &mut Environment) -> RuntimeVal {
    let mut last_evaluated = MK_NULL();
    for statement in program.body {
        last_evaluated = evaluate(statement, env);
    }
    last_evaluated
}

pub fn eval_var_declaration(var_declaration: VarDeclaration, env: &mut Environment) -> RuntimeVal {
    let VarDeclaration { kind: var_decl, constant, identifier, value } = var_declaration;
    let value = match value {
        Some(expr) => evaluate(Stmt::Expr(expr), env),
        None => MK_NULL(),
    };

    env.declare_var(identifier.symbol, value, constant)
}
