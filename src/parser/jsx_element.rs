use crate::{
    ast::{self},
    span::Span,
    token::{Token, TokenKind},
};

use super::{context::ParsingContext, expression};

pub fn parse(ctx: &mut ParsingContext, caller_is_jsx: bool) -> ast::JsxElement {
    if let Token::JsxOpen(name, span) = ctx.get_curr_token() {
        let name = name;
        let span_from = span.from;
        ctx.eat(TokenKind::JsxOpen);

        let attrs = parse_attrs(ctx);
        let mut children = vec![];
        let mut self_closing = false;

        match ctx.get_curr_token() {
            Token::Gt(_) => {
                ctx.eat_jsx(TokenKind::Gt);
                children.append(&mut parse_children(ctx));

                if let Token::JsxClose(closed_name, _) = ctx.get_curr_token() {
                    if closed_name == name {
                        if caller_is_jsx {
                            ctx.eat_jsx(TokenKind::JsxClose);
                        } else {
                            ctx.eat(TokenKind::JsxClose);
                        }
                    } else {
                        ctx.throw_custom("Unexpected closing tag.");
                    }
                }
            }
            Token::JsxSelfClose(_) => {
                self_closing = true;
                if caller_is_jsx {
                    ctx.eat_jsx(TokenKind::JsxSelfClose);
                } else {
                    ctx.eat(TokenKind::JsxSelfClose);
                }
            }
            _ => {
                ctx.throw_unexpected_token();
            }
        }

        let span_to = ctx.get_curr_token().span().from;
        let span = Span::new(span_from, span_to);

        ast::JsxElement {
            name,
            attrs,
            children,
            self_closing,
            span,
        }
    } else {
        ctx.throw_unexpected_token_with_expecting(&TokenKind::JsxOpen);
        panic!()
    }
}

fn parse_attrs(ctx: &mut ParsingContext) -> Vec<ast::JsxElementAttribute> {
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

        let mut expecting_rbrace = false;

        if ctx.get_curr_token().kind() == TokenKind::LBrace {
            ctx.eat(TokenKind::LBrace);
            expecting_rbrace = true;
        }

        let expr = expression::parse(ctx);
        let span_to = ctx.get_prev_token().span().from;

        attrs.push(ast::JsxElementAttribute {
            name,
            expr: expr,
            span: Span::new(span_from, span_to),
        });

        if expecting_rbrace {
            ctx.eat(TokenKind::RBrace);
        }
    }

    attrs
}

fn parse_children(ctx: &mut ParsingContext) -> Vec<ast::Expr> {
    let mut children = vec![];

    while ctx.is_not_eof() {
        if let Token::JsxClose(_, _) = ctx.get_curr_token() {
            break;
        }

        ctx.eat_all_newlines_jsx();

        if ctx.get_curr_token().kind() == TokenKind::LBrace {
            ctx.eat(TokenKind::LBrace);
            children.push(expression::parse(ctx));
            ctx.eat(TokenKind::RBrace);
        } else {
            children.push(expression::parse(ctx));
        }

        if let TokenKind::JsxOpen = TokenKind::from(ctx.get_curr_token()) {
            children.push(ast::Expr::JsxElement(parse(ctx, true)));
        }

        ctx.eat_all_newlines_jsx();
    }

    children
}
