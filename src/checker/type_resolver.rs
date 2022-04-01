use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::checker_context::CheckingContext;

pub fn resolve(ctx: &mut CheckingContext, scope: &Scope, target: &AST) -> String {
    let result = match target {
        AST::Root { children: _ } => "Root".to_string(),
        AST::NumberLiteral { value: _, span: _ } => "Number".to_string(),
        AST::StringLiteral { value: _, span: _ } => "String".to_string(),
        AST::Identifier {
            value,
            generics,
            span: _,
        } => {
            let found_definition = scope.find_definition(value).unwrap();
            let name = resolve(ctx, scope, found_definition);
            let mut generic_names = vec![];
            let mut generics_iter = generics.iter();

            while let Some(generic) = generics_iter.next() {
                generic_names.push(resolve(ctx, scope, generic));
            }

            let name = if generic_names.len() > 0 {
                format!("{}<{}>", name, generic_names.join(","))
            } else {
                name
            };

            name
        }
        AST::JsLiteral { value: _, span: _ } => "JS".to_string(),
        AST::JsxElement {
            name: _,
            attrs: _,
            children: _,
            span: _,
        } => "HtmlElement".to_string(),
        AST::JsxElementAttribute {
            name: _,
            expr: _,
            span: _,
        } => "HtmlElementAttribute".to_string(),
        AST::TypeDefinition {
            name,
            generics: _,
            variants: _,
            span: _,
        } => name.to_string(),
        AST::EnumDefinition {
            name: _,
            generics: _,
            items: _,
            span: _,
        } => "EnumDefinition".to_string(),
        AST::RecordDefinition {
            name: _,
            keys: _,
            span: _,
        } => "RecordDefinition".to_string(),
        AST::RecordKeyDefinition {
            name: _,
            kind: _,
            span: _,
        } => "RecordKeyDefinition".to_string(),
        AST::VariableDeclaration {
            name: _,
            value,
            span: _,
        } => resolve(ctx, scope, &value[0]),
        AST::ArrayDeclarator { items: _, span: _ } => "ArrayDeclarator".to_string(),
        AST::FunctionDefinition {
            name,
            args: _,
            body,
            span: _,
        } => {
            let scope = ctx.get_scope(name).unwrap().clone();

            format!(
                "Fn<{}>",
                resolve(
                    ctx,
                    &scope,
                    body.last().unwrap_or(&AST::Identifier {
                        value: "Unit".into(),
                        generics: vec![],
                        span: SourceSpan::empty(),
                    })
                ),
            )
        }
        AST::FunctionArgumentDefinition {
            name: _,
            type_: kind,
            span: _,
        } => resolve(ctx, scope, kind),
        AST::BinaryExpression {
            kind: _,
            left,
            right: _,
            span: _,
        } => resolve(ctx, scope, &left[0]),
        AST::FunctionCall {
            callee,
            args: _,
            span: _,
        } => resolve(ctx, scope, callee),
        AST::IfStatement {
            test: _,
            consequence: _,
            alternative: _,
            span: _,
        } => "IfStatement".to_string(),
        AST::MemberExpression {
            object: _,
            property: _,
            span: _,
        } => "MemberExpression".to_string(),
        AST::TypeVariant {
            name: _,
            generics: _,
            span: _,
        } => todo!(),
    };

    result.into()
}
