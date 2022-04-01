use crate::{
    ast::AST,
    token::{Token, TokenKind},
};

use super::{context::ParsingContext, function_call, function_definition, type_definition};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    if let Token::Id(value, _) = ctx.get_curr_token() {
        parse_reserved_keywords(ctx, value)
    } else {
        ctx.throw_unexpected_token_with_expecting(&TokenKind::Id);
        panic!()
    }
}

fn parse_reserved_keywords(ctx: &mut ParsingContext, value: String) -> Option<AST> {
    match value.as_str() {
        "type" => type_definition::parse(ctx),
        "fn" => function_definition::parse(ctx),
        _ => parse_non_reserved_keywords(ctx, true),
    }
}

pub fn parse_non_reserved_keywords(ctx: &mut ParsingContext, allow_generics: bool) -> Option<AST> {
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

    let generics = parse_generics(ctx, allow_generics);

    let identifier = AST::Identifier {
        value: current_token.value(),
        generics,
        span,
    };

    let identifier = function_call::parse(ctx, identifier);

    Some(identifier)
}

fn parse_generics(ctx: &mut ParsingContext, allow_generics: bool) -> Vec<AST> {
    let mut generics = vec![];

    if let Token::Lt(_) = ctx.get_curr_token() {
        if !allow_generics {
            ctx.throw_custom("Usage of generic is forbidden here.");
            panic!();
        }

        ctx.eat(TokenKind::Lt);

        while ctx.is_not_eof() {
            if let Token::Gt(_) = ctx.get_curr_token() {
                ctx.eat(TokenKind::Gt);
                break;
            }

            if let Some(generic) = parse_non_reserved_keywords(ctx, true) {
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
