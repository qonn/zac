use crate::ast::AST;

use super::expression;

pub fn generate(ast: &AST) -> String {
    if let AST::VariableStatement {
        name,
        value,
        span: _,
    } = ast
    {
        let value = generate_value(value);

        format!("let {name} = {value}")
    } else {
        "".into()
    }
}

pub fn generate_value(expr: &Vec<AST>) -> String {
    let v = expr
        .iter()
        .map(|expr| match expr {
            AST::ArrayDeclarator { items, span: _ } => generate_array(items),
            _ => expression::generate(expr),
        })
        .collect::<Vec<String>>()
        .join(",");

    v
}

pub fn generate_array(items: &Vec<AST>) -> String {
    let v = items
        .iter()
        .map(|item| expression::generate(item))
        .collect::<Vec<String>>()
        .join(",");

    format!("[{v}]")
}
