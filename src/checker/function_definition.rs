use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
    token::SourceSpan,
};

use super::{
    checker_context::{CheckerContext},
    identifier, statement,
};

pub fn check(context: &mut CheckerContext, scope: &mut Scope, ast: &AST) {
    if let AST::FunctionDefinition {
        name,
        args,
        body,
        span,
    } = ast
    {
        let mut function_scope = scope.clone();
        check_name(context, &mut function_scope, name, span);
        check_args(context, &mut function_scope, args);
        check_body(context, &mut function_scope, body);
        context.add_scope(name, function_scope);
        scope.add_function_definition(name, ast);
    }
}

fn check_name(context: &CheckerContext, scope: &mut Scope, name: &String, span: &SourceSpan) {
    if scope.is_defined(name) {
        context.print_error_message(
            format!(
                "The function name '{}' has already been defined previously.",
                name
            ),
            span.from,
        )
    }
}

fn check_args(context: &CheckerContext, scope: &mut Scope, args: &Vec<AST>) {
    let mut args = args.iter();
    let mut defined_names: Vec<String> = vec![];

    while let Some(arg) = args.next() {
        match arg {
            AST::FunctionArgumentDefinition { name, kind, span } => {
                check_arg_name(context, scope, name, &mut defined_names, arg, span);
                check_arg_kind(context, scope, kind);
                scope.clear_definition_for(name);
                scope.add_variable_definition(name, arg);
            }
            _ => {
                let message = format!("Unexpected function definition argument's syntax.");
                let pos = arg.source_span().from;
                context.print_error_message(message, pos);
            }
        }
    }
}

fn check_arg_name(
    context: &CheckerContext,
    scope: &mut Scope,
    name: &String,
    defined_names: &mut Vec<String>,
    ast: &AST,
    span: &SourceSpan,
) {
    if defined_names.iter().any(|x| x == name) {
        let message = format!("This argument name has been previously defined.");
        let pos = span.from;
        context.print_error_message(message, pos);
    }

    defined_names.push(name.clone());
    scope.add_variable_definition(name, ast);
}

fn check_arg_kind(context: &CheckerContext, scope: &mut Scope, kind: &AST) {
    match ASTKind::from(kind) {
        ASTKind::Identifier => identifier::check(context, scope, kind),
        _ => {
            let message = format!("Unexpected function argument definition type.");
            let pos = kind.source_span().from;
            context.print_error_message(message, pos);
        }
    }
}

fn check_body(context: &CheckerContext, scope: &mut Scope, body: &Vec<AST>) {
    let mut body = body.iter();
    let scope = &mut scope.clone();

    while let Some(body) = body.next() {
        statement::check(context, scope, body);
    }
}
