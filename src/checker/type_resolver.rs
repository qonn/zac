use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::checker_context::CheckerContext;

pub fn resolve(context: &CheckerContext, scope: &Scope, parent: &AST, node: &AST) -> String {
    let result = match node {
        AST::Root { body: _ } => "Root".to_string(),
        AST::NumberLiteral { value: _, span: _ } => "Number".to_string(),
        AST::StringLiteral { value: _, span: _ } => "String".to_string(),
        AST::Identifier {
            value,
            generics: _,
            span: _,
        } => resolve(context, scope, node, scope.find_definition(value).unwrap()),
        AST::JsLiteral { value: _, span: _ } => "JS".to_string(),
        AST::TypeDefinition {
            name: _,
            generics: _,
            items: _,
            span: _,
        } => "TypeDefinition".to_string(),
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
        } => resolve(context, scope, node, &value[0]),
        AST::ArrayDeclarator { items: _, span: _ } => "ArrayDeclarator".to_string(),
        AST::FunctionDefinition {
            name,
            args: _,
            body,
            span: _,
        } => {
            let scope = context.get_scope(name).unwrap();
            resolve(
                context,
                scope,
                node,
                body.last().unwrap_or(&AST::Identifier {
                    value: "Unit".into(),
                    generics: vec![],
                    span: SourceSpan::empty(),
                }),
            )
        }
        AST::FunctionArgumentDefinition {
            name: _,
            kind,
            span: _,
        } => resolve(context, scope, node, kind),
        AST::BinaryExpression {
            kind: _,
            left,
            right: _,
            span: _,
        } => resolve(context, scope, node, &left[0]),
        AST::FunctionCall {
            callee,
            args: _,
            span: _,
        } => resolve(context, scope, node, callee),
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
        AST::BuiltinReservation { span: _ } => {
            if let AST::Identifier {
                value,
                generics: _,
                span: _,
            } = parent
            {
                value.to_string()
            } else {
                "BuiltinReservation".to_string()
            }
        }
    };

    result.into()
}
