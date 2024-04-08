use crate::runtime::values::*;
use crate::frontend::ast::*;
use crate::runtime::environment::*;
use crate::runtime::eval::statements::*;
use crate::runtime::eval::expressions::*;

pub fn evaluate(ast_node: &Stmt, env: &mut Environment) -> RuntimeVal {
    match ast_node {
        Stmt::VarDeclaration(var_declaration) => eval_var_declaration(var_declaration, env),
        Stmt::FunctionDeclaration(function_declaration) => eval_function_declaration(function_declaration, env),
        Stmt::WhileLoop(while_loop) => eval_while_loop(while_loop, env),
        Stmt::ForLoop(for_loop) => eval_for_loop(for_loop, env),
        Stmt::Expr(expr) => eval_expr(expr, env),
        Stmt::Program(program) => eval_program(program, env),
        _ => panic!("Tätä AST-solmua ei ole vielä määritetty tulkittavaksi: {:?}", ast_node),
    }
}

pub fn eval_expr(expr: &Expr, env: &mut Environment) -> RuntimeVal {
    match expr {
        Expr::NumericLiteral(numeric_literal) => { MK_NUMBER(numeric_literal.value) },
        Expr::StringLiteral(string_literal) => eval_string_literal(string_literal, env, &[]),
        Expr::FloatLiteral(float_literal) => { MK_NUMBER(float_literal.value) },
        Expr::Identifier(identifier) => eval_identifier(identifier, env),
        Expr::ObjectLiteral(object_literal) => eval_object_expr(object_literal, env),
        Expr::ArrayLiteral(array_literal) => eval_array_expr(array_literal, env),
        Expr::CallExpr(call_expr) => eval_call_expr(call_expr, env),
        Expr::AssignmentExpr(assignment_expr) => eval_assignment(assignment_expr, env),
        Expr::BinaryExpr(binary_expr) => eval_binary_expr(binary_expr, env),
        Expr::MemberExpr(member_expr) => eval_member_expr(member_expr, env),
        Expr::UnaryExpr(unary_expr) => eval_unary_expr(unary_expr, env),
        Expr::IfElseExpr(if_else_expr) => eval_if_else_expr(if_else_expr, env),
        _ => panic!("Tätä AST-solmua ei ole vielä määritetty tulkittavaksi: {:?}", expr),
    }
}
