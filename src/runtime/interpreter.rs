use crate::runtime::values::*;
use crate::frontend::ast::*;
use crate::runtime::environment::*;
use crate::runtime::eval::statements::*;
use crate::runtime::eval::expressions::*;

pub fn evaluate(ast_node: Stmt, env: &mut Environment) -> RuntimeVal {
    match ast_node {
        Stmt::VarDeclaration(var_declaration) => eval_var_declaration(var_declaration, env),
        Stmt::Expr(expr) => {
            match expr {
                Expr::NumericLiteral(numeric_literal) => {
                    RuntimeVal::Number(NumberVal::mk_number(numeric_literal.value))
                }
                Expr::Identifier(identifier) => eval_identifier(identifier, env),
                Expr::ObjectLiteral(object_literal) => eval_object_expr(object_literal, env),
                Expr::AssignmentExpr(assignment_expr) => eval_assignment(assignment_expr, env),
                Expr::BinaryExpr(binary_expr) => eval_binary_expr(binary_expr, env),
                _ => panic!("This AST Node has not yet been setup for interpretation: {:?}", expr),
            }
        }
        Stmt::Program(program) => eval_program(program, env),
        _ => panic!("This AST Node has not yet been setup for interpretation: {:?}", ast_node),
    }
}
