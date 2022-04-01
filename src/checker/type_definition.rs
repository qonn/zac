use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::{checker_context::CheckingContext, identifier};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::TypeDefinition {
        name,
        generics,
        variants: items,
        span,
    } = ast
    {
        check_name(ctx, scope, name, span);

        let mut item_iters = items.iter();

        while let Some(ast) = item_iters.next() {
            check_variants(ctx, scope, generics, ast);
        }
    }
}

pub fn check_name(ctx: &mut CheckingContext, scope: &mut Scope, name: &String, span: &SourceSpan) {
    if let Some(other) = scope.get_type_definition(name) {
        let message = format!("The type '{}' has been previously defined.", name);
        let pos = span.from;
        ctx.print_error_message(message, pos);

        let message = format!("It was previously defined here.");
        let pos = other.source_span().from;
        ctx.print_error_message(message, pos);

        panic!();
    }
}

pub fn check_variants(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    type_definition_generics: &Vec<AST>,
    item: &AST,
) {
    match item {
        AST::Identifier {
            value: _,
            generics: _,
            span: _,
        } => {
            identifier::check(ctx, scope, item);
        }
        AST::FunctionCall {
            callee: _,
            args: _,
            span: _,
        } => check(ctx, scope, item),
        AST::TypeVariant {
            name,
            generics,
            span: _,
        } => check_variant(ctx, scope, type_definition_generics, name, generics),
        _ => {
            let message = format!("Unsupported type definition syntax.");
            let pos = item.source_span().from;
            ctx.print_error_message(message, pos);
        }
    }
}

fn check_variant(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    type_definition_generics: &Vec<AST>,
    name: &String,
    generics: &Vec<AST>,
) {
    let mut generics_iter = generics.iter();

    while let Some(AST::Identifier {
        value: _,
        generics: _,
        span: _,
    }) = generics_iter.next()
    {}
}
