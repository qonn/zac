use crate::{
    ast::AST,
    token::{SourceSpan, Token, TokenKind},
};

use super::{context::ParsingContext, expression};

pub fn parse(ctx: &mut ParsingContext, identifier: AST) -> AST {
    if let Token::LParen(_) = ctx.get_curr_token() {
        let span_from = identifier.source_span().from;

        ctx.eat(TokenKind::LParen);

        let args = parse_args(ctx);

        ctx.eat_all_newlines();

        let span_to = ctx.get_curr_token().span().to;

        ctx.eat(TokenKind::RParen);

        AST::FunctionCall {
            callee: Box::new(identifier),
            args,
            span: SourceSpan::new(span_from, span_to),
        }
    } else {
        identifier
    }
}

fn parse_args(ctx: &mut ParsingContext) -> Vec<AST> {
    let mut args = vec![];

    while ctx.is_not_eof() {
        if let Token::RParen(_) = ctx.get_curr_token() {
            break;
        }

        ctx.eat_all_newlines();

        if let Some(arg) = expression::parse(ctx) {
            args.push(arg);
        }

        if let Token::Comma(_) = ctx.get_curr_token() {
            ctx.eat(TokenKind::Comma);
        }

        ctx.eat_all_newlines();
    }

    args
}
