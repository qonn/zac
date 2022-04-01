mod context;
mod root;
mod statement;
mod identifier;
mod string_literal;
mod number_literal;
mod js_literal;
mod type_definition;
mod function_definition;
mod type_variant;
mod jsx_element;
mod expression;
mod function_call;
mod function_anon_definition;

use self::context::ParsingContext;
use crate::{ast::AST, lexer::Lexer};

pub fn parse(lexer: &mut Lexer) -> AST {
    let mut context = ParsingContext::new(lexer);
    let ast = root::parse(&mut context);
    ast
}
