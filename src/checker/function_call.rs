use super::{binary_expression, checker_context::CheckerContext, identifier, type_resolver};
use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
};

pub fn check(context: &CheckerContext, scope: &mut Scope, ast: &AST) {
    if let AST::FunctionCall {
        callee,
        args,
        span: _,
    } = ast
    {
        let caller = ast;

        if let Some(callee) = resolve_callee(context, scope, callee) {
            check_args(context, scope, caller, &callee, args);
        }
    }
}

fn resolve_callee<'a>(context: &CheckerContext, scope: &'a Scope, callee: &AST) -> Option<AST> {
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
            context.print_error_message(message, pos);
            None
        }
    } else {
        None
    }
}

fn check_args(
    context: &CheckerContext,
    scope: &mut Scope,
    caller: &AST,
    callee: &AST,
    args: &Vec<AST>,
) {
    if let AST::FunctionDefinition {
        name: _,
        body: _,
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
            context.print_error_message(message, pos);
        }

        let mut callee_args = callee_args.iter();
        let mut caller_args = args.iter();

        while let (Some(callee_arg), Some(caller_arg)) = (callee_args.next(), caller_args.next()) {
            match ASTKind::from(caller_arg) {
                ASTKind::Identifier => {
                    identifier::check(context, scope, caller_arg);
                }
                ASTKind::NumberLiteral => {}
                ASTKind::StringLiteral => {}
                ASTKind::FunctionCall => check(context, scope, caller_arg),
                ASTKind::BinaryExpression => binary_expression::check(context, scope, caller_arg),
                _ => {
                    let message = format!("Unexpected function call argument's syntax.");
                    let pos = caller_arg.source_span().from;
                    context.print_error_message(message, pos);
                }
            }

            check_arg_type(context, scope, callee, callee_arg, caller, caller_arg);
        }
    }
}

fn check_arg_type(
    context: &CheckerContext,
    scope: &Scope,
    callee: &AST,
    callee_arg: &AST,
    caller: &AST,
    caller_arg: &AST,
) {
    let resolved_callee_arg = type_resolver::resolve(context, scope, callee, callee_arg);
    let resolved_caller_arg = type_resolver::resolve(context, scope, caller, caller_arg);

    if let (
        AST::FunctionDefinition {
            name: callee_name,
            span: _,
            args: _,
            body: _,
        },
        AST::FunctionArgumentDefinition {
            name: callee_arg_name,
            kind: _,
            span: _,
        },
    ) = (callee, callee_arg)
    {
        if resolved_callee_arg != resolved_caller_arg {
            let message = format!(
                "The function '{}', argument '{}' was expecting '{}' but received a '{}'.",
                callee_name, callee_arg_name, resolved_callee_arg, resolved_caller_arg
            );
            let pos = caller_arg.source_span().from;
            context.print_error_message(message, pos);
        }
    }
}
