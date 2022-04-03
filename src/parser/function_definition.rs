use crate::{
    ast::AST,
    token::{SourceSpan, Token, TokenKind},
};

use super::{context::ParsingContext, identifier, statement};

pub fn parse(ctx: &mut ParsingContext) -> AST {
    let span_from = ctx.get_curr_token().span().from;

    ctx.eat(TokenKind::Id); // the 'fn' keyword

    let name = ctx.get_curr_token().value();

    ctx.eat(TokenKind::Id);

    ctx.eat(TokenKind::LParen);

    let args = parse_args(ctx);

    ctx.eat(TokenKind::RParen);

    ctx.eat(TokenKind::LBrace);

    let body = parse_body(ctx);

    ctx.eat(TokenKind::RBrace);

    let span_to = ctx.get_curr_token().span().from;

    AST::FunctionDefinition {
        name,
        args,
        body,
        span: SourceSpan::new(span_from, span_to),
    }
}

fn parse_args(ctx: &mut ParsingContext) -> Vec<AST> {
    let mut args = vec![];

    ctx.eat_all_newlines();

    while ctx.is_not_eof() {
        ctx.eat_all_newlines();

        if let Token::RParen(_) = ctx.get_curr_token() {
            break;
        }

        let span_from = ctx.get_curr_token().span().from;

        let name = ctx.get_curr_token().value();
        ctx.eat(TokenKind::Id);

        ctx.eat(TokenKind::DblColon);

        let type_ = Box::new(identifier::parse_non_reserved_keywords(ctx, true));

        let span_to = ctx.get_curr_token().span().from;

        args.push(AST::FunctionArgumentDefinition {
            name,
            type_,
            span: SourceSpan::new(span_from, span_to),
        });

        if let Token::Comma(_) = ctx.get_curr_token() {
            ctx.eat(TokenKind::Comma);
        }
    }

    args
}

fn parse_body(ctx: &mut ParsingContext) -> Vec<AST> {
    let mut body = vec![];

    while ctx.is_not_eof() {
        ctx.eat_all_newlines();

        if let Token::RBrace(_) = ctx.get_curr_token() {
            break;
        }

        if let Some(ast) = statement::parse(ctx) {
            body.push(ast);
        }
    }
    body
}
