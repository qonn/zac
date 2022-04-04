use super::{
    binary_expression, context::CheckingContext, function_definition, identifier, jsx_element,
    type_resolver,
};
use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::FunctionCall {
        callee,
        args,
        span: _,
    } = ast
    {
        let caller = ast;

        if let Some(callee) = resolve_callee(ctx, scope, callee) {
            check_args(ctx, scope, caller, &callee, args);
        }
    }
}

fn resolve_callee(ctx: &mut CheckingContext, scope: &mut Scope, callee: &AST) -> Option<AST> {
    if let AST::Identifier {
        value,
        generics: _,
        span,
    } = callee
    {
        if let Some(ast) = scope.get_function_definition(value) {
            Some(ast.clone())
        } else if let Some(ast) = scope.get_variable_definition(value) {
            Some(ast.clone())
        } else {
            let message = format!("The function '{:?}' used here could not be found.", value);
            let pos = span.from;
            ctx.print_error_message(message, pos);
            None
        }
    } else {
        None
    }
}

fn check_args(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    caller: &AST,
    callee: &AST,
    args: &Vec<AST>,
) {
    if let AST::FunctionDefinition {
        name: _,
        body: _,
        expected_return_type: _,
        args: callee_args,
        span: _,
    } = callee
    {
        if callee_args.len() != args.len() {
            let message = format!(
                "This function takes {} arguments but {} were specified",
                callee_args.len(),
                args.len()
            );
            let pos = caller.source_span().from;
            ctx.print_error_message(message, pos);
        }

        let mut callee_args = callee_args.iter();
        let mut caller_args = args.iter();

        while let (Some(callee_arg), Some(caller_arg)) = (callee_args.next(), caller_args.next()) {
            match ASTKind::from(caller_arg) {
                ASTKind::Identifier => {
                    identifier::check(ctx, scope, caller_arg);
                }
                ASTKind::NumberLiteral => {}
                ASTKind::StringLiteral => {}
                ASTKind::JsLiteral => {}
                ASTKind::FunctionCall => check(ctx, scope, caller_arg),
                ASTKind::FunctionDefinition => function_definition::check(ctx, scope, caller_arg),
                ASTKind::JsxElement => jsx_element::check(ctx, scope, caller_arg),
                ASTKind::BinaryExpression => binary_expression::check(ctx, scope, caller_arg),
                _ => {
                    let message = format!("Unexpected function call argument's syntax.");
                    let pos = caller_arg.source_span().from;
                    ctx.print_error_message(message, pos);
                }
            }

            check_arg_type(ctx, scope, callee, callee_arg, caller_arg);
        }
    }
}

fn check_arg_type(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    callee: &AST,
    callee_arg: &AST,
    caller_arg: &AST,
) {
    let resolved_callee_arg = type_resolver::resolve(ctx, scope, callee_arg);
    let resolved_caller_arg = type_resolver::resolve(ctx, scope, caller_arg);

    if let (
        AST::FunctionDefinition {
            name: callee_name,
            expected_return_type: _,
            args: _,
            body: _,
            span: _,
        },
        AST::FunctionArgumentDefinition {
            name: callee_arg_name,
            type_: _,
            span: _,
        },
    ) = (callee, callee_arg)
    {
        if resolved_callee_arg != resolved_caller_arg && resolved_caller_arg != "JS" {
            let message = format!(
                "The function '{}', argument '{}' was expecting '{}' but received a '{}'.",
                callee_name, callee_arg_name, resolved_callee_arg, resolved_caller_arg
            );
            let pos = caller_arg.source_span().from;
            ctx.print_error_message(message, pos);
        }
    }
}
