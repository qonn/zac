use crate::{
    ast,
    span::Span,
    token::{Token, TokenKind},
};

use super::{
    context::ParsingContext, function_call, identifier, literal_js, member_access, statement_if,
    statement_let, statement_return,
};

pub fn parse(ctx: &mut ParsingContext, anonymous: bool) -> ast::Fn {
    let span_from = ctx.get_curr_token().span().from;

    if !anonymous {
        ctx.eat(TokenKind::Fn);
    }

    let id = if !anonymous {
        identifier::parse(ctx)
    } else {
        ast::Ident {
            string: ctx.get_new_anon_name(),
            generics: vec![],
            span: Span::new(
                ctx.get_curr_token().span().from,
                ctx.get_curr_token().span().from,
            ),
        }
    };

    ctx.eat(TokenKind::LParen);

    let args = parse_args(ctx);

    ctx.eat(TokenKind::RParen);

    let output;

    if ctx.get_curr_token().kind() == TokenKind::DblColon {
        ctx.eat(TokenKind::DblColon);
        output = ast::Type::Ident(identifier::parse(ctx));
    } else {
        output = ast::Type::Default;
    }

    ctx.eat(TokenKind::LBrace);

    let stmts = parse_statements(ctx);

    ctx.eat(TokenKind::RBrace);

    let span_to = ctx.get_curr_token().span().from;

    ast::Fn {
        args,
        id,
        anonymous,
        stmts,
        output,
        span: Span::new(span_from, span_to),
    }
}

fn parse_args(ctx: &mut ParsingContext) -> Vec<ast::FnArg> {
    let mut args = vec![];

    ctx.eat_all_newlines();

    while ctx.is_not_eof() {
        ctx.eat_all_newlines();

        if let Token::RParen(_) = ctx.get_curr_token() {
            break;
        }

        let span_from = ctx.get_curr_token().span().from;

        let id = identifier::parse(ctx);

        let input = if ctx.get_curr_token().kind() == TokenKind::DblColon {
            ctx.eat(TokenKind::DblColon);
            ast::Type::Ident(identifier::parse(ctx))
        } else {
            ast::Type::Default
        };

        let span_to = ctx.get_curr_token().span().from;

        args.push(ast::FnArg {
            span: Span::new(span_from, span_to),
            id,
            input,
        });

        if let Token::Comma(_) = ctx.get_curr_token() {
            ctx.eat(TokenKind::Comma);
        }
    }

    args
}

fn parse_statements(ctx: &mut ParsingContext) -> Vec<ast::FnStmt> {
    let mut body = vec![];

    while ctx.is_not_eof() {
        ctx.eat_all_newlines();

        if let Token::RBrace(_) = ctx.get_curr_token() {
            break;
        }

        body.push(parse_statement(ctx));
    }

    body
}

fn parse_statement(ctx: &mut ParsingContext) -> ast::FnStmt {
    let fn_stmt = match ctx.get_curr_token().kind() {
        TokenKind::Let => ast::FnStmt::Let(statement_let::parse(ctx)),
        TokenKind::Id => {
            let id = identifier::parse(ctx);

            if ctx.peek_ahead_ignoring_newlines().kind() == TokenKind::Dot {
                ctx.eat_all_newlines();
            }

            match ctx.get_curr_token().kind() {
                TokenKind::LBrace => ast::FnStmt::FnCall(function_call::parse(ctx, id)),
                TokenKind::Dot => {
                    ast::FnStmt::MemberAccess(member_access::parse(ctx, ast::Expr::Id(id)))
                }
                _ => {
                    ctx.throw_unexpected_token();
                    panic!()
                }
            }
        }
        TokenKind::If => ast::FnStmt::If(statement_if::parse(ctx)),
        TokenKind::Return => ast::FnStmt::Ret(statement_return::parse(ctx)),
        TokenKind::Js => ast::FnStmt::LitJs(literal_js::parse(ctx)),
        TokenKind::NewLine => ast::FnStmt::Noop,
        _ => {
            ctx.throw_unexpected_token();
            panic!()
        }
    };

    fn_stmt
}
