use crate::ast;

use super::{
    binary, fn_call, identifier, init_array, jsx_element, literal_js, literal_number,
    literal_string, member_access, statement_fn, statement_if,
};
use super::{context, literal_boolean};

pub fn generate(ctx: &mut context::Context, ast: &ast::Expr) -> String {
    match ast {
        ast::Expr::Id(v) => identifier::generate(v),
        ast::Expr::Fn(v) => statement_fn::generate(ctx, v),
        ast::Expr::FnCall(v) => fn_call::generate(ctx, v),
        ast::Expr::Binary(v) => binary::generate(ctx, v),
        ast::Expr::LitBoolean(v) => literal_boolean::generate(v),
        ast::Expr::LitNumber(v) => literal_number::generate(v),
        ast::Expr::LitString(v) => literal_string::generate(ctx, v, false, false),
        ast::Expr::LitJs(v) => literal_js::generate(ctx, v),
        ast::Expr::JsxElement(v) => jsx_element::generate(ctx, v),
        ast::Expr::InitArray(v) => init_array::generate(ctx, v),
        ast::Expr::InitRecord(v) => todo!(),
        ast::Expr::MemberAccess(v) => member_access::generate(ctx, v),
        ast::Expr::If(v) => statement_if::generate(ctx, v),
    }
}
