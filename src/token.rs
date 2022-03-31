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
