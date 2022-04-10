use crate::{
    ast,
    token::{Token, TokenKind},
};

use super::context::ParsingContext;

pub fn parse(ctx: &mut ParsingContext) -> ast::Ident {
    let current_token = ctx.get_curr_token();

    match current_token.value().as_str() {
        "type" => {
            ctx.throw_reserved_keywords("type");
        }
        "fn" => {
            ctx.throw_reserved_keywords("fn");
        }
        _ => {}
    }

    let span = current_token.span().clone();

    ctx.eat_without_consuming_jsx(TokenKind::Id);

    let generics = parse_generics(ctx);

    let identifier = ast::Ident {
        string: current_token.value(),
        generics,
        span,
    };

    identifier
}

fn parse_generics(ctx: &mut ParsingContext) -> Vec<ast::Ident> {
    let mut generics = vec![];

    if let Token::Lt(_) = ctx.get_curr_token() {
        ctx.eat(TokenKind::Lt);

        while ctx.is_not_eof() {
            if let Token::Gt(_) = ctx.get_curr_token() {
                ctx.eat(TokenKind::Gt);
                break;
            }

            generics.push(parse(ctx));

            if let Token::Comma(_) = ctx.get_curr_token() {
                ctx.eat(TokenKind::Comma);
            }

            if let Token::DblColon(_) = ctx.get_curr_token() {
                ctx.throw_unexpected_token()
            }
        }
    }

    generics
}
