use strum_macros::EnumDiscriminants;

use crate::span::Span;

#[derive(Clone, Debug, EnumDiscriminants)]
#[strum_discriminants(name(TokenKind))]
pub enum Token {
    Mod(Span),
    If(Span),
    Fn(Span),
    Id(String, Span),
    Str(String, Span),
    Numeric(String, Span),
    Boolean(String, Span),
    Js(String, Span),
    JsxOpen(String, Span),
    JsxSelfClose(Span),
    JsxClose(String, Span),
    Plus(Span),
    Minus(Span),
    Divide(Span),
    Multiply(Span),
    Dot(Span),
    Eq(Span),
    LParen(Span),
    RParen(Span),
    LBrace(Span),
    RBrace(Span),
    LSqrBr(Span),
    RSqrBr(Span),
    Gt(Span),
    Lt(Span),
    DblColon(Span),
    Comma(Span),
    NewLine(Span),
    Return(Span),
    Let(Span),
    Eof(Span),
}

impl Token {
    pub fn value(&self) -> String {
        match self {
            Token::Mod(_) => String::from("mod"),
            Token::If(_) => String::from("if"),
            Token::Fn(_) => String::from("fn"),
            Token::Id(v, _) => v.clone(),
            Token::Str(v, _) => v.clone(),
            Token::Numeric(v, _) => v.clone(),
            Token::Boolean(v, _) => v.clone(),
            Token::Js(v, _) => v.clone(),
            Token::JsxOpen(v, _) => v.clone(),
            Token::JsxSelfClose(_) => String::from("/>"),
            Token::JsxClose(v, _) => v.clone(),
            Token::Plus(_) => String::from("+"),
            Token::Minus(_) => String::from("-"),
            Token::Divide(_) => String::from("/"),
            Token::Multiply(_) => String::from("*"),
            Token::Eq(_) => String::from("="),
            Token::LParen(_) => String::from("("),
            Token::RParen(_) => String::from(")"),
            Token::LBrace(_) => String::from("{"),
            Token::RBrace(_) => String::from("}"),
            Token::Gt(_) => String::from(">"),
            Token::Lt(_) => String::from("<"),
            Token::DblColon(_) => String::from(":"),
            Token::Comma(_) => String::from(","),
            Token::NewLine(_) => String::from("\n"),
            Token::Eof(_) => String::from(""),
            Token::LSqrBr(_) => String::from("["),
            Token::RSqrBr(_) => String::from("]"),
            Token::Dot(_) => String::from("."),
            Token::Return(_) => String::from("return"),
            Token::Let(_) => String::from("let"),
        }
    }

    pub fn span(&self) -> Span {
        let s = match self {
            Token::Mod(s) => s,
            Token::Fn(s) => s,
            Token::If(s) => s,
            Token::Id(_, s) => s,
            Token::Str(_, s) => s,
            Token::Numeric(_, s) => s,
            Token::Boolean(_, s) => s,
            Token::Js(_, s) => s,
            Token::JsxOpen(_, s) => s,
            Token::JsxSelfClose(s) => s,
            Token::JsxClose(_, s) => s,
            Token::Plus(s) => s,
            Token::Minus(s) => s,
            Token::Divide(s) => s,
            Token::Multiply(s) => s,
            Token::Eq(s) => s,
            Token::LParen(s) => s,
            Token::RParen(s) => s,
            Token::LBrace(s) => s,
            Token::RBrace(s) => s,
            Token::Gt(s) => s,
            Token::Lt(s) => s,
            Token::DblColon(s) => s,
            Token::Comma(s) => s,
            Token::NewLine(s) => s,
            Token::Eof(s) => s,
            Token::LSqrBr(s) => s,
            Token::RSqrBr(s) => s,
            Token::Dot(s) => s,
            Token::Return(s) => s,
            Token::Let(s) => s,
        };

        s.clone()
    }

    pub fn kind(&self) -> TokenKind {
        TokenKind::from(self)
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::Eof(Span::new(0, 0))
    }
}
