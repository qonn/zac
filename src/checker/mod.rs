mod checker_context;
mod enum_definition;
mod identifier;
mod record_definition;
mod root;
mod type_definition;
mod variable_declaration;
mod function_call;
mod function_definition;
mod array_declarator;
mod statement;
mod literal;
mod if_statement;
mod binary_expression;
mod type_resolver;

use crate::ast::AST;

pub fn check(filepath: &String, content: &String, ast: Vec<AST>) {
    let mut context = checker_context::new(filepath, content);
    root::check(&mut context, ast);
}
