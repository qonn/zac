use crate::{
    ast::AST,
    token::{SourceSpan, Token, TokenKind},
};

use super::{context::ParsingContext, expression};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    if let Token::JsxOpen(name, span) = ctx.get_curr_token() {
        let name = name;
        let span_from = span.from;
        ctx.eat(TokenKind::JsxOpen);

        let attrs = parse_attrs(ctx);
        let mut children = vec![];

        match ctx.get_curr_token() {
            Token::Gt(_) => {
                ctx.eat_jsx(TokenKind::Gt);
                children.append(&mut parse_children(ctx));

                if let Token::JsxClose(closed_name, _) = ctx.get_curr_token() {
                    if closed_name == name {
                        ctx.eat_jsx(TokenKind::JsxClose);
                    } else {
                        ctx.throw_custom("Unexpected closing tag.");
                    }
                }
            }
            Token::JsxSelfClose(_) => ctx.eat_jsx(TokenKind::JsxSelfClose),
            _ => {
                ctx.throw_unexpected_token();
            }
        }

        let span_to = ctx.get_curr_token().span().from;
        let span = SourceSpan::new(span_from, span_to);

        Some(AST::JsxElement {
            name,
            attrs,
            children,
            span,
        })
    } else {
        ctx.throw_unexpected_token_with_expecting(&TokenKind::JsxOpen);
        None
    }
}

fn parse_attrs(ctx: &mut ParsingContext) -> Vec<AST> {
    let mut attrs = vec![];

    while ctx.is_not_eof() {
        if let Token::JsxSelfClose(_) = ctx.get_curr_token() {
            break;
        }

        if let Token::Gt(_) = ctx.get_curr_token() {
            break;
        }

        let span_from = ctx.get_curr_token().span().from;

        let name = ctx.get_curr_token().value();

        ctx.eat(TokenKind::Id);
        ctx.eat(TokenKind::Eq);

        if let Some(expr) = expression::parse(ctx) {
            let span_to = ctx.get_prev_token().span().from;

            attrs.push(AST::JsxElementAttribute {
                name,
                expr: Box::new(expr),
                span: SourceSpan::new(span_from, span_to),
            })
        }
    }

    attrs
}

fn parse_children(ctx: &mut ParsingContext) -> Vec<AST> {
    let mut children = vec![];

    while ctx.is_not_eof() {
        if let Token::JsxClose(_, _) = ctx.get_curr_token() {
            break;
        }

        if let Some(token) = expression::parse(ctx) {
            children.push(token);
        }

        if let TokenKind::JsxOpen = TokenKind::from(ctx.get_curr_token()) {
            if let Some(token) = parse(ctx) {
                children.push(token);
            }
        }

        ctx.eat_all_newlines_jsx();
    }

    children
}
