#[derive(Debug, Clone)]
pub struct SourceSpan {
    pub line: usize,
    pub from: usize,
    pub to: usize,
}

impl SourceSpan {
    pub fn new(line: usize, from: usize, to: usize) -> SourceSpan {
        SourceSpan { line, from, to }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Id { span: SourceSpan, content: String },
    Str { span: SourceSpan, content: String },
    Numeric { span: SourceSpan, content: String },
    Plus { span: SourceSpan },
    Minus { span: SourceSpan },
    Divide { span: SourceSpan },
    Multiply { span: SourceSpan },
    Eq { span: SourceSpan },
    LParen { span: SourceSpan },
    RParen { span: SourceSpan },
    LBrace { span: SourceSpan },
    RBrace { span: SourceSpan },
    DblColon { span: SourceSpan },
    Comma { span: SourceSpan },
    NewLine { span: SourceSpan },
    Js { span: SourceSpan, content: String },
}
