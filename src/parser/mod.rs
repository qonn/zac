mod context;
mod expression;
mod function;
mod function_call;
mod identifier;
mod statement_if;
mod literal_js;
mod jsx_element;
mod statement_let;
mod member_access;
mod literal_number;
mod statement_return;
mod root;
mod statement;
mod literal_string;
mod array;
mod statement_module;
mod statements;
mod literal_boolean;

use self::context::ParsingContext;
use crate::{ast, lexer::Lexer};

pub fn parse(lexer: &mut Lexer) -> ast::Root {
    let mut context = ParsingContext::new(lexer);
    let ast = root::parse(&mut context);
    ast
}
