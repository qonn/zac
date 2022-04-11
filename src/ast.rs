use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
pub struct Root {
    pub name: String,
    pub path: String,
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

impl Spanned for Root {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Mod(Mod),
    Let(Let),
    Enum(Enum),
    Record(Record),
    Fn(Fn),
    FnCall(FnCall),
    MemberAccess(MemberAccess),
    Return(Return),
    LitJs(LitJs),
    Noop,
}

impl From<Expr> for Stmt {
    fn from(val: Expr) -> Self {
        match val {
            Expr::Fn(v) => Stmt::Fn(v),
            Expr::FnCall(v) => Stmt::FnCall(v),
            Expr::MemberAccess(v) => Stmt::MemberAccess(*v),
            _ => panic!("Unsupported expression to statement"),
        }
    }
}
impl From<Mod> for Stmt {
    fn from(val: Mod) -> Self {
        Stmt::Mod(val)
    }
}

impl From<Let> for Stmt {
    fn from(val: Let) -> Self {
        Stmt::Let(val)
    }
}

impl From<Enum> for Stmt {
    fn from(val: Enum) -> Self {
        Stmt::Enum(val)
    }
}

impl From<Record> for Stmt {
    fn from(val: Record) -> Self {
        Stmt::Record(val)
    }
}

impl From<Fn> for Stmt {
    fn from(val: Fn) -> Self {
        Stmt::Fn(val)
    }
}

impl From<FnCall> for Stmt {
    fn from(val: FnCall) -> Self {
        Stmt::FnCall(val)
    }
}

impl From<MemberAccess> for Stmt {
    fn from(val: MemberAccess) -> Self {
        Stmt::MemberAccess(val)
    }
}

impl From<Return> for Stmt {
    fn from(val: Return) -> Self {
        Stmt::Return(val)
    }
}

impl From<LitJs> for Stmt {
    fn from(val: LitJs) -> Self {
        Stmt::LitJs(val)
    }
}

impl Spanned for Stmt {
    fn span(&self) -> Span {
        match self {
            Stmt::Mod(v) => v.span(),
            Stmt::Let(v) => v.span(),
            Stmt::Enum(v) => v.span(),
            Stmt::Record(v) => v.span(),
            Stmt::Fn(v) => v.span(),
            Stmt::FnCall(v) => v.span(),
            Stmt::MemberAccess(v) => v.span(),
            Stmt::Return(v) => v.span(),
            Stmt::LitJs(v) => v.span(),
            Stmt::Noop => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: Ident,
    pub props: Vec<RecProp>,
    pub span: Span,
}

impl Spanned for Record {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct RecProp {
    pub id: Ident,
    pub init: Type,
    pub span: Span,
}

impl Spanned for RecProp {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub id: Ident,
    pub span: Span,
}

impl Spanned for Enum {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Mod {
    pub path: String,
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

impl Spanned for Mod {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Let {
    pub id: String,
    pub expr: Expr,
    pub span: Span,
}
impl Spanned for Let {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Id(Ident),
    Fn(Fn),
    FnCall(FnCall),
    Binary(Box<Binary>),
    LitBoolean(LitBoolean),
    LitNumber(LitNumber),
    LitString(LitString),
    LitJs(LitJs),
    JsxElement(JsxElement),
    InitArray(InitArray),
    InitRecord(InitRecord),
    MemberAccess(Box<MemberAccess>),
    If(Box<If>),
}

impl From<Ident> for Expr {
    fn from(val: Ident) -> Self {
        Expr::Id(val)
    }
}

impl From<Fn> for Expr {
    fn from(val: Fn) -> Self {
        Expr::Fn(val)
    }
}

impl From<FnCall> for Expr {
    fn from(val: FnCall) -> Self {
        Expr::FnCall(val)
    }
}

impl From<Binary> for Expr {
    fn from(val: Binary) -> Self {
        Expr::Binary(Box::new(val))
    }
}

impl From<LitBoolean> for Expr {
    fn from(val: LitBoolean) -> Self {
        Expr::LitBoolean(val)
    }
}

impl From<LitNumber> for Expr {
    fn from(val: LitNumber) -> Self {
        Expr::LitNumber(val)
    }
}

impl From<LitString> for Expr {
    fn from(val: LitString) -> Self {
        Expr::LitString(val)
    }
}

impl From<LitJs> for Expr {
    fn from(val: LitJs) -> Self {
        Expr::LitJs(val)
    }
}

impl From<JsxElement> for Expr {
    fn from(val: JsxElement) -> Self {
        Expr::JsxElement(val)
    }
}

impl From<InitArray> for Expr {
    fn from(val: InitArray) -> Self {
        Expr::InitArray(val)
    }
}

impl From<InitRecord> for Expr {
    fn from(val: InitRecord) -> Self {
        Expr::InitRecord(val)
    }
}

impl From<If> for Expr {
    fn from(val: If) -> Self {
        Expr::If(Box::new(val))
    }
}

impl From<MemberAccess> for Expr {
    fn from(val: MemberAccess) -> Self {
        Expr::MemberAccess(Box::new(val))
    }
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Id(v) => v.span(),
            Expr::Fn(v) => v.span(),
            Expr::FnCall(v) => v.span(),
            Expr::Binary(v) => v.span(),
            Expr::LitBoolean(v) => v.span(),
            Expr::LitNumber(v) => v.span(),
            Expr::LitString(v) => v.span(),
            Expr::LitJs(v) => v.span(),
            Expr::JsxElement(v) => v.span(),
            Expr::InitArray(v) => v.span(),
            Expr::InitRecord(v) => v.span(),
            Expr::MemberAccess(v) => v.span(),
            Expr::If(v) => v.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LitBoolean {
    pub raw: String,
    pub value: bool,
    pub span: Span,
}

impl Spanned for LitBoolean {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct LitNumber {
    pub raw: String,
    pub value: f64,
    pub span: Span,
}

impl Spanned for LitNumber {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct LitString {
    pub raw: String,
    pub value: String,
    pub span: Span,
}

impl Spanned for LitString {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct LitJs {
    pub raw: String,
    pub value: String,
    pub span: Span,
}

impl Spanned for LitJs {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct JsxElement {
    pub name: String,
    pub attrs: Vec<JsxElementAttribute>,
    pub children: Vec<Expr>,
    pub self_closing: bool,
    pub span: Span,
}

impl Spanned for JsxElement {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct JsxElementAttribute {
    pub name: String,
    pub expr: Expr,
    pub span: Span,
}

impl Spanned for JsxElementAttribute {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Fn {
    pub id: Ident,
    pub anonymous: bool,
    pub args: Vec<FnArg>,
    pub stmts: Vec<FnStmt>,
    pub output: Type,
    pub span: Span,
}

impl Spanned for Fn {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FnArg {
    pub id: Ident,
    pub input: Type,
    pub span: Span,
}

impl Spanned for FnArg {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub id: Ident,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl Spanned for FnCall {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Default,
    Ident(Ident),
}

#[derive(Debug, Clone)]
pub enum FnStmt {
    Let(Let),
    FnCall(FnCall),
    MemberAccess(MemberAccess),
    LitJs(LitJs),
    If(If),
    Ret(Return),
    Noop,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub expr: Expr,
    pub span: Span,
}

impl Spanned for Return {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub op: BinaryOp,
    pub left: Expr,
    pub right: Expr,
    pub span: Span,
}

impl Spanned for Binary {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    Gt,
}

impl BinaryOp {
    pub(crate) fn value(&self) -> String {
        match self {
            BinaryOp::Add => "+".into(),
            BinaryOp::Sub => "-".into(),
            BinaryOp::Mul => "*".into(),
            BinaryOp::Div => "/".into(),
            BinaryOp::Eq => "=".into(),
            BinaryOp::Lt => "<".into(),
            BinaryOp::Gt => ">".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub string: String,
    pub generics: Vec<Ident>,
    pub span: Span,
}
impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InitArray {
    pub items: Vec<Expr>,
    pub span: Span,
}

impl Spanned for InitArray {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InitRecord {
    pub props: Vec<InitObjectProp>,
    pub span: Span,
}

impl Spanned for InitRecord {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub enum InitObjectProp {
    Key(Ident),
    KeyValue(Ident, Expr),
    Spread(Ident),
}

#[derive(Debug, Clone)]
pub struct If {
    pub test: Expr,
    pub truthy: Expr,
    pub falsy: Expr,
    pub span: Span,
}

impl Spanned for If {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct MemberAccess {
    pub obj: Expr,
    pub prop: Expr,
    pub span: Span,
}

impl Spanned for MemberAccess {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
impl Ident {
    pub fn concat(&self, other: Ident) -> Ident {
        let string = vec![self.string.clone(), other.string.clone()].join(".");

        Ident {
            string,
            generics: other.generics,
            span: Span::new(self.span.from, other.span.to),
        }
    }
}
