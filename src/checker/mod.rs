mod array_declarator;
mod binary_expression;
mod checker_context;
mod enum_definition;
mod function_call;
mod function_definition;
mod identifier;
mod if_statement;
mod literal;
mod member_expression;
mod record_definition;
mod root;
mod statement;
mod type_definition;
mod type_resolver;
mod variable_declaration;
mod jsx_element;

use crate::ast::AST;

pub fn check(filepath: &String, content: &String, ast: AST) {
    let mut context = checker_context::new(filepath, content);
    root::check(&mut context, ast);
}
