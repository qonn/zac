use crate::{ast, token::TokenKind};

use super::{
    context::ParsingContext, expression, function, function_call, identifier, literal_js,
    member_access, statement_let, statement_module, statement_return,
};

pub fn parse(ctx: &mut ParsingContext) -> ast::Stmt {
    let token = ctx.get_curr_token();

    let statement: ast::Stmt = match TokenKind::from(&token) {
        TokenKind::Mod => ast::Stmt::Mod(statement_module::parse(ctx)),
        TokenKind::Fn => ast::Stmt::Fn(function::parse(ctx, false)),
        TokenKind::Let => ast::Stmt::Let(statement_let::parse(ctx)),
        TokenKind::Return => ast::Stmt::Return(statement_return::parse(ctx)),
        TokenKind::Js => ast::Stmt::LitJs(literal_js::parse(ctx)),
        TokenKind::Id => {
            let id = identifier::parse(ctx);

            if ctx.peek_ahead_ignoring_newlines().kind() == TokenKind::Dot {
                ctx.eat_all_newlines();
            }

            match ctx.get_curr_token().kind() {
                TokenKind::LParen => function_call::parse(ctx, id).into(),
                TokenKind::Dot => member_access::parse(ctx, ast::Expr::Id(id)).into(),

                _ => {
                    ctx.throw_unexpected_token();
                    panic!();
                }
            }
        }
        TokenKind::NewLine => {
            ctx.eat(TokenKind::NewLine);
            ast::Stmt::Noop
        }
        _ => {
            ctx.throw_unexpected_token();
            panic!()
        }
    };

    statement
}
