#[derive(Debug, Clone)]
pub struct Span {
    pub from: usize,
    pub to: usize,
}

impl Span {
    pub fn new(from: usize, to: usize) -> Span {
        Span { from, to }
    }

    pub fn empty() -> Span {
        Span { from: 0, to: 0 }
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}
