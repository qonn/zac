use crate::{
    ast::ASTKind,
    ast::AST,
    scope::{self, Scope},
};

use super::{
    checker_context::CheckerContext, enum_definition, function_call, function_definition,
    record_definition, type_definition, variable_declaration,
};

pub fn check(context: &mut CheckerContext, ast: Vec<AST>) {
    let mut ast_iter = ast.iter();
    let mut global_scope = scope::new();

    while let Some(ast) = ast_iter.next() {
        match ast {
            AST::Root { body } => {
                check_body(context, &mut global_scope, body);
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
}

pub fn check_body(context: &mut CheckerContext, scope: &mut Scope, body: &Vec<AST>) {
    let mut body_iter = body.iter();

    while let Some(body) = body_iter.next() {
        match body {
            AST::Root { body: _ } => {}
            AST::NumberLiteral { value: _, span: _ } => {}
            AST::StringLiteral { value: _, span: _ } => {}
            AST::Identifier {
                value: _,
                generics: _,
                span: _,
            } => {}
            AST::JsLiteral { value: _, span: _ } => {}
            AST::TypeDefinition {
                name,
                generics: _,
                items: _,
                span: _,
            } => {
                type_definition::check(context, scope, body);
                scope.add_type_definition(&name, body);
            }
            AST::EnumDefinition {
                name,
                generics: _,
                items: _,
                span: _,
            } => {
                enum_definition::check(context, scope, body);
                scope.add_enum_definition(&name, body);
            }
            AST::RecordDefinition {
                name,
                keys: _,
                span: _,
            } => {
                record_definition::check(context, scope, body);
                scope.add_record_definition(&name, body);
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
                variable_declaration::check(context, scope, body);
                scope.add_variable_definition(&name, body);
            }
            AST::ArrayDeclarator { items: _, span: _ } => {}
            AST::FunctionDefinition {
                name: _,
                args: _,
                body: _,
                span: _,
            } => {
                function_definition::check(context, scope, body);
            }
            AST::FunctionArgumentDefinition {
                name: _,
                kind: _,
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
                function_call::check(context, scope, body);
            }
            AST::IfStatement {
                test: _,
                consequence: _,
                alternative: _,
                span: _,
            } => {}
            AST::BuiltinReservation { span: _ } => {}
            AST::MemberExpression {
                object,
                property,
                span,
            } => {}
        }
    }
}
