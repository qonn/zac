use crate::{
    ast::AST,
    token::{Token, TokenKind},
};

use super::{context::ParsingContext};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    if let Token::Id(_, _) = ctx.get_curr_token() {
        parse_non_reserved_keywords(ctx)
    } else {
        ctx.throw_unexpected_token_with_expecting(&TokenKind::Id);
        panic!()
    }
}

pub fn parse_non_reserved_keywords(ctx: &mut ParsingContext) -> Option<AST> {
    let current_token = ctx.get_curr_token();
    let span = current_token.span().clone();

    ctx.eat(TokenKind::Id);

    let generics = parse_generics(ctx);

    Some(AST::TypeVariant {
        name: current_token.value(),
        generics,
        span,
    })
}

fn parse_generics(ctx: &mut ParsingContext) -> Vec<AST> {
    let mut generics = vec![];

    if let Token::LParen(_) = ctx.get_curr_token() {
        ctx.eat(TokenKind::LParen);

        while ctx.is_not_eof() {
            if let Token::RParen(_) = ctx.get_curr_token() {
                ctx.eat(TokenKind::RParen);
                break;
            }

            if let Some(generic) = parse_non_reserved_keywords(ctx) {
                generics.push(generic)
            }

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
