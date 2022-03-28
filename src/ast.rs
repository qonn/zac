use strum_macros::EnumDiscriminants;

#[derive(Debug, Clone)]
pub enum ASTBinaryExpressionKind {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(name(ASTKind))]
pub enum AST {
    Root {
        body: Vec<AST>,
    },
    NumberLiteral {
        value: String,
    },
    StringLiteral {
        value: String,
    },
    Identifier {
        value: String,
        generics: Vec<AST>,
    },
    JsLiteral {
        value: String,
    },
    TypeDefinition {
        name: String,
        generics: Vec<AST>,
        items: Vec<AST>,
    },
    EnumDefinition {
        name: String,
        generics: Vec<AST>,
        items: Vec<AST>,
    },
    RecordDefinition {
        name: String,
        keys: Vec<AST>,
    },
    RecordKeyDefinition {
        name: String,
        kind: Vec<AST>,
    },
    VariableDeclaration {
        name: String,
        value: Vec<AST>,
    },
    ArrayDeclarator {
        items: Vec<AST>,
    },
    FunctionDefinition {
        name: String,
        args: Vec<AST>,
        body: Vec<AST>,
    },
    FunctionArgumentDefinition {
        name: String,
        kind: String,
    },
    BinaryExpression {
        kind: ASTBinaryExpressionKind,
        left: Vec<AST>,
        right: Vec<AST>,
    },
    FunctionCall {
        callee: Box<AST>,
        args: Vec<AST>,
    },
    IfStatement {
        test: Vec<AST>,
        consequence: Vec<AST>,
        alternative: Vec<AST>,
    },
    BuiltinReservation,
}
