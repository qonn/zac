use super::{context::CheckingContext, function_definition, type_resolver};
use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
    token::SourceSpan,
};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::JsxElement {
        name,
        attrs,
        children,
        self_closing,
        span,
    } = ast
    {
        check_name(ctx, scope, name, span);
        check_children(ctx, scope, children);
    } else {
    }
}
fn check_name(ctx: &mut CheckingContext, scope: &mut Scope, name: &String, span: &SourceSpan) {
    if !scope.is_defined(name) && !is_reserved_name(name) {
        let message = format!("This JSX Element '{}' used here could not be found.", name);
        let pos = span.from;
        ctx.print_error_message(message, pos);
    }

    if let Some(t) = scope.find_definition(name) {
        let resolved_type = function_definition::resolve_returning_type(ctx, scope, t);
        if resolved_type != "Element" {
            let message = format!(
                "This JSX Element '{}' used here is not a valid JSX Element",
                name
            );
            let pos = span.from;
            ctx.print_error_message(message, pos);
        }
    }
}

fn is_reserved_name(name: &String) -> bool {
    match name.as_str() {
        "a" | "abbr" | "address" | "area" | "article" | "aside" | "audio" | "b" | "base"
        | "bdi" | "bdo" | "blockquote" | "body" | "br" | "button" | "canvas" | "caption"
        | "cite" | "code" | "col" | "colgroup" | "data" | "datalist" | "dd" | "del" | "details"
        | "dfn" | "dialog" | "div" | "dl" | "dt" | "em" | "embed" | "fieldset" | "figcaption"
        | "figure" | "footer" | "form" | "head" | "header" | "hgroup" | "h1" | "h2" | "h3"
        | "h4" | "h5" | "h6" | "hr" | "html" | "i" | "iframe" | "img" | "input" | "ins" | "kbd"
        | "keygen" | "label" | "legend" | "li" | "link" | "main" | "map" | "mark" | "menu"
        | "menuitem" | "meta" | "meter" | "nav" | "noscript" | "object" | "ol" | "optgroup"
        | "option" | "output" | "p" | "param" | "picture" | "pre" | "progress" | "q" | "rp"
        | "rt" | "ruby" | "s" | "samp" | "script" | "section" | "select" | "small" | "source"
        | "span" | "strong" | "style" | "sub" | "summary" | "sup" | "svg" | "table" | "tbody"
        | "td" | "template" | "textarea" | "tfoot" | "th" | "thead" | "time" | "title" | "tr"
        | "track" | "u" | "ul" | "var" | "video" | "wbr" => true,
        _ => false,
    }
}

fn check_children(ctx: &mut CheckingContext, scope: &mut Scope, children: &[AST]) {
    let mut children_iter = children.iter();

    while let Some(child) = children_iter.next() {
        match ASTKind::from(child) {
            ASTKind::JsxElement => check(ctx, scope, child),
            _ => {}
        }
    }
}
