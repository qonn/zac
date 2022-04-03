use crate::ast::AST;
use crate::token::{SourceSpan, TokenKind};

use super::context::ParsingContext;
use super::{expression, identifier, number_literal, string_literal};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    let span_from = ctx.get_curr_token().span().from;

    ctx.eat(TokenKind::Let);

    let name = ctx.get_curr_token().value();

    ctx.eat(TokenKind::Id);
    ctx.eat(TokenKind::Eq);
    ctx.eat_all_newlines();

    let value = vec![parse_declarator(ctx)];

    let span_to = ctx.get_curr_token().span().from;

    Some(AST::VariableStatement {
        name,
        value,
        span: SourceSpan::new(span_from, span_to),
    })
}

pub fn parse_declarator(ctx: &mut ParsingContext) -> AST {
    match TokenKind::from(ctx.get_curr_token()) {
        TokenKind::LSqrBr => parse_array_declarator(ctx),
        TokenKind::Id => identifier::parse(ctx),
        TokenKind::Str => string_literal::parse(ctx),
        TokenKind::Numeric => number_literal::parse(ctx),
        _ => {
            ctx.throw_unexpected_token();
            panic!()
        }
    }
}

fn parse_array_declarator(ctx: &mut ParsingContext) -> AST {
    let span_from = ctx.get_curr_token().span().from;

    ctx.eat(TokenKind::LSqrBr);
    ctx.eat_all_newlines();

    let mut items = vec![];

    while ctx.is_not_eof() {
        if ctx.get_curr_token().kind() == TokenKind::RSqrBr {
            break;
        }

        if let Some(item) = parse_array_declarator_item(ctx) {
            items.push(item);
        } else {
            ctx.throw_custom("Unexpected expression here.")
        }
    }

    ctx.eat(TokenKind::RSqrBr);

    let span_to = ctx.get_prev_token().span().to;

    AST::ArrayDeclarator {
        items,
        span: SourceSpan::new(span_from, span_to),
    }
}

fn parse_array_declarator_item(ctx: &mut ParsingContext) -> Option<AST> {
    let ast = expression::parse(ctx);

    ctx.eat_all_newlines();

    if ctx.get_curr_token().kind() != TokenKind::RSqrBr {
        ctx.eat(TokenKind::Comma);
    }

    ctx.eat_all_newlines();

    return ast;
}
