use strum_macros::EnumDiscriminants;

#[derive(Debug, Clone)]
pub struct SourceSpan {
    pub from: usize,
    pub to: usize,
}

impl SourceSpan {
    pub fn new(from: usize, to: usize) -> SourceSpan {
        SourceSpan { from, to }
    }

    pub fn empty() -> SourceSpan {
        SourceSpan { from: 0, to: 0 }
    }
}

#[derive(Clone, Debug, EnumDiscriminants)]
#[strum_discriminants(name(TokenKind))]
pub enum Token {
    Id(String, SourceSpan),
    Str(String, SourceSpan),
    Numeric(String, SourceSpan),
    Js(String, SourceSpan),
    JsxOpen(String, SourceSpan),
    JsxSelfClose(SourceSpan),
    JsxClose(String, SourceSpan),
    Plus(SourceSpan),
    Minus(SourceSpan),
    Divide(SourceSpan),
    Multiply(SourceSpan),
    Dot(SourceSpan),
    Eq(SourceSpan),
    LParen(SourceSpan),
    RParen(SourceSpan),
    LBrace(SourceSpan),
    RBrace(SourceSpan),
    LSqrBr(SourceSpan),
    RSqrBr(SourceSpan),
    Gt(SourceSpan),
    Lt(SourceSpan),
    DblColon(SourceSpan),
    Comma(SourceSpan),
    NewLine(SourceSpan),
    Eof(SourceSpan),
}

impl Token {
    pub fn value(&self) -> String {
        match self {
            Token::Id(v, _) => v.clone(),
            Token::Str(v, _) => v.clone(),
            Token::Numeric(v, _) => v.clone(),
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
        }
    }

    pub fn span(&self) -> SourceSpan {
        let ss = match self {
            Token::Id(_, ss) => ss,
            Token::Str(_, ss) => ss,
            Token::Numeric(_, ss) => ss,
            Token::Js(_, ss) => ss,
            Token::JsxOpen(_, ss) => ss,
            Token::JsxSelfClose(ss) => ss,
            Token::JsxClose(_, ss) => ss,
            Token::Plus(ss) => ss,
            Token::Minus(ss) => ss,
            Token::Divide(ss) => ss,
            Token::Multiply(ss) => ss,
            Token::Eq(ss) => ss,
            Token::LParen(ss) => ss,
            Token::RParen(ss) => ss,
            Token::LBrace(ss) => ss,
            Token::RBrace(ss) => ss,
            Token::Gt(ss) => ss,
            Token::Lt(ss) => ss,
            Token::DblColon(ss) => ss,
            Token::Comma(ss) => ss,
            Token::NewLine(ss) => ss,
            Token::Eof(ss) => ss,
            Token::LSqrBr(ss) => ss,
            Token::RSqrBr(ss) => ss,
            Token::Dot(ss) => ss,
        };

        ss.clone()
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::Eof(SourceSpan::new(0, 0))
    }
}
