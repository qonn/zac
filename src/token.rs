use strum_macros::EnumDiscriminants;

#[derive(Debug, Clone)]
pub struct SourceSpan {
    pub line: usize,
    pub from: usize,
    pub to: usize,
}

impl SourceSpan {
    pub fn new(row: usize, from: usize, to: usize) -> SourceSpan {
        SourceSpan {
            line: row,
            from,
            to,
        }
    }

    pub fn empty() -> SourceSpan {
        SourceSpan {
            line: 0,
            from: 0,
            to: 0,
        }
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
    Eq(String, SourceSpan),
    LParen(String, SourceSpan),
    RParen(String, SourceSpan),
    LBrace(String, SourceSpan),
    RBrace(String, SourceSpan),
    Gt(String, SourceSpan),
    Lt(String, SourceSpan),
    DblColon(String, SourceSpan),
    Comma(String, SourceSpan),
    NewLine(String, SourceSpan),
    Js(String, SourceSpan),
    Unknown(String, SourceSpan),
}

impl Token {
    pub fn unknown() -> Token {
        Token::Unknown("".into(), SourceSpan::empty())
    }
}
