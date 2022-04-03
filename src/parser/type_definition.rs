use crate::{
    ast::AST,
    token::{SourceSpan, Token, TokenKind},
};

use super::{context::ParsingContext, identifier, type_variant};

pub fn parse(ctx: &mut ParsingContext) -> AST {
    ctx.eat(TokenKind::Id); // the 'type' keyword

    let token = ctx.get_curr_token();

    let from = token.span().from;
    let name = token.value().clone();

    ctx.eat_without_consuming_jsx(TokenKind::Id);

    let generics = parse_generics(ctx);
    let variants = parse_variants(ctx);
    let to = ctx.get_curr_token().span().from;

    AST::TypeDefinition {
        name,
        generics,
        variants,
        span: SourceSpan::new(from, to),
    }
}

fn parse_generics(ctx: &mut ParsingContext) -> Vec<AST> {
    let mut generics = vec![];

    if let Token::Lt(_) = ctx.get_curr_token() {
        ctx.eat(TokenKind::Lt);

        while ctx.is_not_eof() {
            if let Token::Gt(_) = ctx.get_curr_token() {
                ctx.eat(TokenKind::Gt);
                break;
            }

            generics.push(identifier::parse(ctx));

            if let Token::Comma(_) = ctx.get_curr_token() {
                ctx.eat(TokenKind::Comma);
            }
        }
    }

    generics
}

fn parse_variants(ctx: &mut ParsingContext) -> Vec<AST> {
    let mut variants = vec![];

    if let Token::LBrace(_) = ctx.get_curr_token() {
        ctx.eat(TokenKind::LBrace);

        while ctx.is_not_eof() {
            ctx.eat_all_newlines();

            if let Token::RBrace(_) = ctx.get_curr_token() {
                ctx.eat(TokenKind::RBrace);
                break;
            }

            if let Some(variant) = type_variant::parse(ctx) {
                variants.push(variant)
            }

            ctx.eat_all_newlines();
            ctx.eat(TokenKind::Comma);
        }
    }

    variants
}
