mod array_declarator;
mod binary_expression;
pub mod context;
mod enum_definition;
mod function_call;
mod function_definition;
mod identifier;
mod if_statement;
mod jsx_element;
mod literal;
mod member_expression;
mod record_definition;
mod root;
mod statement;
mod type_definition;
mod type_resolver;
mod variable_declaration;
mod return_statement;

use crate::ast::AST;

pub fn check(filepath: &String, content: &String, ast: &AST) {
    let mut context = context::new(filepath, content);
    root::check(&mut context, ast.clone());
}
