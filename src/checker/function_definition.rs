use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
    token::SourceSpan,
};

use super::{context::CheckingContext, identifier, statement};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::FunctionDefinition {
        name,
        args,
        body,
        span,
    } = ast
    {
        let mut function_scope = scope.clone();
        check_name(ctx, &mut function_scope, name, span);
        check_args(ctx, &mut function_scope, args);
        check_body(ctx, &mut function_scope, body);
        ctx.add_scope(name, function_scope);
        scope.add_function_definition(name, ast);
    }
}

fn check_name(ctx: &mut CheckingContext, scope: &mut Scope, name: &String, span: &SourceSpan) {
    if scope.is_defined(name) {
        ctx.print_error_message(
            format!(
                "The function name '{}' has already been defined previously.",
                name
            ),
            span.from,
        )
    }
}

fn check_args(ctx: &mut CheckingContext, scope: &mut Scope, args: &Vec<AST>) {
    let mut args = args.iter();
    let mut defined_names: Vec<String> = vec![];

    while let Some(arg) = args.next() {
        match arg {
            AST::FunctionArgumentDefinition {
                name,
                type_: kind,
                span,
            } => {
                check_arg_name(ctx, scope, name, &mut defined_names, arg, span);
                check_arg_kind(ctx, scope, kind);
                scope.clear_definition_for(name);
                scope.add_variable_definition(name, arg);
            }
            _ => {
                let message = format!("Unexpected function definition argument's syntax.");
                let pos = arg.source_span().from;
                ctx.print_error_message(message, pos);
            }
        }
    }
}

fn check_arg_name(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    name: &String,
    defined_names: &mut Vec<String>,
    ast: &AST,
    span: &SourceSpan,
) {
    if defined_names.iter().any(|x| x == name) {
        let message = format!("This argument name has been previously defined.");
        let pos = span.from;
        ctx.print_error_message(message, pos);
    }

    defined_names.push(name.clone());
    scope.add_variable_definition(name, ast);
}

fn check_arg_kind(ctx: &mut CheckingContext, scope: &mut Scope, kind: &AST) {
    match ASTKind::from(kind) {
        ASTKind::Identifier => identifier::check(ctx, scope, kind),
        _ => {
            let message = format!("Unexpected function argument definition type.");
            let pos = kind.source_span().from;
            ctx.print_error_message(message, pos);
        }
    }
}

fn check_body(ctx: &mut CheckingContext, scope: &mut Scope, body: &Vec<AST>) {
    let mut body = body.iter();
    let scope = &mut scope.clone();

    while let Some(body) = body.next() {
        statement::check(ctx, scope, body);
    }
}
