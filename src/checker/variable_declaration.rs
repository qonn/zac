use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::{array_declarator, context::CheckingContext, function_call};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::VariableStatement { value, name, span } = ast {
        check_name(ctx, span, scope, name);

        let mut value_iters = value.iter();

        while let Some(ast) = value_iters.next() {
            check_value(ctx, scope, ast);
        }

        scope.add_variable_definition(name, ast);
    }
}

pub fn check_name(ctx: &mut CheckingContext, span: &SourceSpan, scope: &mut Scope, name: &String) {
    if let Some(other) = scope.get_type_definition(name) {
        let message = format!("This variable name '{}' has been previously defined.", name);
        let pos = span.from;
        ctx.print_error_message(message, pos);

        let message = format!("It was previously defined here.");
        let pos = other.source_span().from;
        ctx.print_error_message(message, pos);

        panic!();
    }
}

pub fn check_value(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    match ast {
        AST::NumberLiteral { value: _, span: _ } => {}
        AST::StringLiteral { value: _, span: _ } => {}
        AST::JsLiteral { value: _, span: _ } => {}
        AST::ArrayDeclarator { items: _, span: _ } => {
            array_declarator::check(ctx, scope, ast);
        }
        AST::FunctionCall {
            callee: _,
            args: _,
            span: _,
        } => function_call::check(ctx, scope, ast),
        AST::BinaryExpression {
            kind: _,
            left: _,
            right: _,
            span: _,
        } => {}
        AST::MemberExpression {
            object: _,
            property: _,
            span: _,
        } => {}
        AST::IfStatement {
            test: _,
            consequence: _,
            alternative: _,
            span: _,
        } => {}
        _ => {
            let message = format!("Unsupported variable declaration syntax.");
            let pos = ast.source_span().from;
            ctx.print_error_message(message, pos);
        }
    }
}
