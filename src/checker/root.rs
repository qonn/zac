use crate::{
    ast::ASTKind,
    ast::AST,
    scope::{self, Scope},
};

use super::{
    array_declarator, context::CheckingContext, enum_definition, function_call,
    function_definition, member_expression, record_definition, type_definition,
    variable_declaration,
};

pub fn check(ctx: &mut CheckingContext, ast: AST) {
    let mut global_scope = scope::new();

    match ast {
        AST::Root { children } => {
            check_children(ctx, &mut global_scope, &children);
        }
        _ => {
            panic!(
                "Unexpected '{:?}', expecting '{:?}'",
                ASTKind::from(ast),
                ASTKind::Root
            )
        }
    }
}

pub fn check_children(ctx: &mut CheckingContext, scope: &mut Scope, body: &Vec<AST>) {
    let mut children_iter = body.iter();

    while let Some(child) = children_iter.next() {
        match child {
            AST::Root { children: _ } => {}
            AST::NumberLiteral { value: _, span: _ } => {}
            AST::StringLiteral { value: _, span: _ } => {}
            AST::Identifier {
                value: _,
                generics: _,
                span: _,
            } => {}
            AST::JsLiteral { value: _, span: _ } => {}
            AST::JsxElement {
                name: _,
                attrs: _,
                children: _,
                self_closing: _,
                span: _,
            } => {}
            AST::JsxElementAttribute {
                name: _,
                expr: _,
                span: _,
            } => {}
            AST::TypeDefinition {
                name,
                generics: _,
                variants: _,
                span: _,
            } => {
                type_definition::check(ctx, scope, child);
                scope.add_type_definition(&name, child);
            }
            AST::EnumDefinition {
                name,
                generics: _,
                items: _,
                span: _,
            } => {
                enum_definition::check(ctx, scope, child);
                scope.add_enum_definition(&name, child);
            }
            AST::RecordDefinition {
                name,
                keys: _,
                span: _,
            } => {
                record_definition::check(ctx, scope, child);
                scope.add_record_definition(&name, child);
            }
            AST::RecordKeyDefinition {
                name: _,
                kind: _,
                span: _,
            } => {}
            AST::VariableDeclaration {
                name,
                value: _,
                span: _,
            } => {
                variable_declaration::check(ctx, scope, child);
                scope.add_variable_definition(&name, child);
            }
            AST::ArrayDeclarator { items: _, span: _ } => {
                array_declarator::check(ctx, scope, child);
            }
            AST::FunctionDefinition {
                name: _,
                args: _,
                body: _,
                span: _,
            } => {
                function_definition::check(ctx, scope, child);
            }
            AST::FunctionArgumentDefinition {
                name: _,
                type_: _,
                span: _,
            } => {}
            AST::BinaryExpression {
                kind,
                left,
                right,
                span,
            } => {}
            AST::FunctionCall {
                callee: _,
                args: _,
                span: _,
            } => {
                function_call::check(ctx, scope, child);
            }
            AST::IfStatement {
                test: _,
                consequence: _,
                alternative: _,
                span: _,
            } => {}
            AST::MemberExpression {
                object,
                property,
                span,
            } => member_expression::check(ctx, scope, child),
            AST::TypeVariant {
                name,
                generics,
                span,
            } => {}
        }
    }
}
