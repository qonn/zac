use super::{
    binary_expression, checker_context::CheckerContext, function_call, identifier, if_statement,
    literal, variable_declaration,
};
use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
};

pub fn check(context: &CheckerContext, scope: &mut Scope, ast: &AST) {
    match ASTKind::from(ast) {
        ASTKind::JsLiteral => literal::check(context, scope, ast),
        ASTKind::StringLiteral => literal::check(context, scope, ast),
        ASTKind::NumberLiteral => literal::check(context, scope, ast),
        ASTKind::IfStatement => if_statement::check(context, scope, ast),
        ASTKind::BinaryExpression => binary_expression::check(context, scope, ast),
        ASTKind::VariableDeclaration => variable_declaration::check(context, scope, ast),
        ASTKind::FunctionCall => function_call::check(context, scope, ast),
        _ => {
            let message = format!("Unsupported '{:?}' statement.", ASTKind::from(ast));
            let pos = ast.source_span().from;
            context.print_error_message(message, pos);
            return;
        }
    }
}
