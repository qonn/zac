use super::context;
use crate::{ast, generator::expression};

pub fn generate(ctx: &mut context::Context, v: &ast::InitArray) -> String {
    let items = v
        .items
        .iter()
        .map(|x| expression::generate(ctx, x))
        .collect::<Vec<_>>()
        .join(",");

    format!("[{items}]")
}
