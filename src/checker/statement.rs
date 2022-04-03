use super::{
    binary_expression, context::CheckingContext, function_call, if_statement, jsx_element, literal,
    return_statement, variable_declaration,
};
use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    match ASTKind::from(ast) {
        ASTKind::JsLiteral => literal::check(ctx, scope, ast),
        ASTKind::StringLiteral => literal::check(ctx, scope, ast),
        ASTKind::NumberLiteral => literal::check(ctx, scope, ast),
        ASTKind::IfStatement => if_statement::check(ctx, scope, ast),
        ASTKind::BinaryExpression => binary_expression::check(ctx, scope, ast),
        ASTKind::VariableStatement => variable_declaration::check(ctx, scope, ast),
        ASTKind::FunctionCall => function_call::check(ctx, scope, ast),
        ASTKind::ReturnStatement => return_statement::check(ctx, scope, ast),
        ASTKind::JsxElement => jsx_element::check(ctx, scope, ast),
        _ => {
            let message = format!("Unsupported '{:?}' statement.", ASTKind::from(ast));
            let pos = ast.source_span().from;
            ctx.print_error_message(message, pos);
            return;
        }
    }
}
