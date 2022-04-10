use crate::{ast::AST, scope::Scope, token::Span};

use super::{context::CheckingContext, statement, type_resolver::resolve};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::FunctionDefinition {
        name,
        expected_return_type: _,
        args,
        body,
        span,
    } = ast
    {
        let mut function_scope = scope.clone();
        function_scope.owner = Some(ast.clone());
        check_name(ctx, &mut function_scope, name, span);
        check_args(ctx, &mut function_scope, args);
        check_body(ctx, &mut function_scope, body);
        ctx.add_scope(name, function_scope);
        scope.add_function_definition(name, ast);
    }
}

fn check_name(ctx: &mut CheckingContext, scope: &mut Scope, name: &String, span: &Span) {
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
                type_: _,
                span,
            } => {
                check_arg_name(ctx, scope, name, &mut defined_names, arg, span);
                scope.clear_definition_for(name);
                scope.add_variable_definition(name, arg);
            }
            _ => {
                let message = format!("Unexpected function definition argument's syntax.");
                let pos = arg.span().from;
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
    span: &Span,
) {
    if defined_names.iter().any(|x| x == name) {
        let message = format!("This argument name has been previously defined.");
        let pos = span.from;
        ctx.print_error_message(message, pos);
    }

    defined_names.push(name.clone());
    scope.add_variable_definition(name, ast);
}

fn check_body(ctx: &mut CheckingContext, scope: &mut Scope, body: &Vec<AST>) {
    let mut body = body.iter();
    let scope = &mut scope.clone();

    while let Some(body) = body.next() {
        statement::check(ctx, scope, body);
    }
}

pub fn resolve_returning_type(ctx: &mut CheckingContext, scope: &Scope, function: &AST) -> String {
    let default_return_type = &AST::Identifier {
        value: "Unit".into(),
        generics: vec![],
        span: Span::empty(),
    };

    if let AST::FunctionDefinition {
        name: _,
        args: _,
        expected_return_type,
        body,
        span: _,
    } = function
    {
        if let Some(expected_return_type) = expected_return_type {
            resolve(ctx, scope, expected_return_type)
        } else {
            resolve(ctx, &scope, body.last().unwrap_or(default_return_type))
        }
    } else {
        resolve(ctx, scope, default_return_type)
    }
}
