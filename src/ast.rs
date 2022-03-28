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
    BlockStatement {
        body: Vec<AST>,
    },
    Identifier {
        value: String,
    },
    JsLiteral {
        value: String,
    },
    TypeDefinition {
        name: String,
        value: Vec<AST>,
    },
    VariableDefinition {
        name: String,
        value: Vec<AST>,
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
        alternative: Vec<AST>
    },
    Noop,
}
