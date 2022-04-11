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
    let mut string = current_token.value();

    ctx.eat_without_consuming_jsx(TokenKind::Id);

    while ctx.get_curr_token().kind() == TokenKind::Dot {
        ctx.eat(TokenKind::Dot);
        let string_part = ctx.get_curr_token().value();
        string = format!("{string}.{string_part}");
        ctx.eat_without_consuming_jsx(TokenKind::Id);
    }

    let generics = parse_generics(ctx);

    let identifier = ast::Ident {
        string,
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
