use strum_macros::EnumDiscriminants;

use crate::token::SourceSpan;

#[derive(Debug, Clone)]
pub enum ASTBinaryExpressionKind {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(ASTKind))]
pub enum AST {
    Root {
        body: Vec<AST>,
    },
    NumberLiteral {
        value: String,
        span: SourceSpan,
    },
    StringLiteral {
        value: String,
        span: SourceSpan,
    },
    Identifier {
        value: String,
        generics: Vec<AST>,
        span: SourceSpan,
    },
    JsLiteral {
        value: String,
        span: SourceSpan,
    },
    TypeDefinition {
        name: String,
        generics: Vec<AST>,
        items: Vec<AST>,
        span: SourceSpan,
    },
    EnumDefinition {
        name: String,
        generics: Vec<AST>,
        items: Vec<AST>,
        span: SourceSpan,
    },
    RecordDefinition {
        name: String,
        keys: Vec<AST>,
        span: SourceSpan,
    },
    RecordKeyDefinition {
        name: String,
        kind: Vec<AST>,
        span: SourceSpan,
    },
    VariableDeclaration {
        name: String,
        value: Vec<AST>,
        span: SourceSpan,
    },
    ArrayDeclarator {
        items: Vec<AST>,
        span: SourceSpan,
    },
    FunctionDefinition {
        name: String,
        args: Vec<AST>,
        body: Vec<AST>,
        span: SourceSpan,
    },
    FunctionArgumentDefinition {
        name: String,
        kind: Box<AST>,
        span: SourceSpan,
    },
    BinaryExpression {
        kind: ASTBinaryExpressionKind,
        left: Vec<AST>,
        right: Vec<AST>,
        span: SourceSpan,
    },
    FunctionCall {
        callee: Box<AST>,
        args: Vec<AST>,
        span: SourceSpan,
    },
    IfStatement {
        test: Vec<AST>,
        consequence: Vec<AST>,
        alternative: Vec<AST>,
        span: SourceSpan,
    },
    MemberExpression {
        object: Box<AST>,
        property: Box<AST>,
        span: SourceSpan,
    },
    BuiltinReservation {
        span: SourceSpan,
    },
}

impl AST {
    pub fn source_span(&self) -> SourceSpan {
        let span = SourceSpan::new(0, 0);

        let span = match self {
            AST::Root { body: _ } => &span,
            AST::NumberLiteral { value: _, span } => span,
            AST::StringLiteral { value: _, span } => span,
            AST::Identifier {
                value: _,
                generics: _,
                span,
            } => span,
            AST::JsLiteral { value: _, span } => span,
            AST::TypeDefinition {
                name: _,
                generics: _,
                items: _,
                span,
            } => span,
            AST::EnumDefinition {
                name: _,
                generics: _,
                items: _,
                span,
            } => span,
            AST::RecordDefinition {
                name: _,
                keys: _,
                span,
            } => span,
            AST::RecordKeyDefinition {
                name: _,
                kind: _,
                span,
            } => span,
            AST::VariableDeclaration {
                name: _,
                value: _,
                span,
            } => span,
            AST::ArrayDeclarator { items: _, span } => span,
            AST::FunctionDefinition {
                name: _,
                args: _,
                body: _,
                span,
            } => span,
            AST::FunctionArgumentDefinition {
                name: _,
                kind: _,
                span,
            } => span,
            AST::BinaryExpression {
                kind: _,
                left: _,
                right: _,
                span,
            } => span,
            AST::FunctionCall {
                callee: _,
                args: _,
                span,
            } => span,
            AST::IfStatement {
                test: _,
                consequence: _,
                alternative: _,
                span,
            } => span,

            AST::BuiltinReservation { span } => span,
            AST::MemberExpression {
                object: _,
                property: _,
                span,
            } => span,
        };

        span.clone()
    }
}
