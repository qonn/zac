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
    Plus(String, SourceSpan),
    Minus(String, SourceSpan),
    Divide(String, SourceSpan),
    Multiply(String, SourceSpan),
    Dot(SourceSpan),
    Eq(String, SourceSpan),
    LParen(String, SourceSpan),
    RParen(String, SourceSpan),
    LBrace(String, SourceSpan),
    RBrace(String, SourceSpan),
    LSqrBr(SourceSpan),
    RSqrBr(SourceSpan),
    Gt(String, SourceSpan),
    Lt(String, SourceSpan),
    DblColon(String, SourceSpan),
    Comma(String, SourceSpan),
    NewLine(String, SourceSpan),
    Js(String, SourceSpan),
    Eof(SourceSpan),
}
