use crate::{
    ast::{ASTBinaryExpressionKind, AST},
    error_message::ErrorMessage,
    lexer::Lexer,
    token::{SourceSpan, Token, TokenKind},
};

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub previous_token: Option<Token>,
    pub current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        let current_token = lexer.get_next_token();
        let previous_token = current_token.clone();

        Parser {
            lexer,
            current_token,
            previous_token,
        }
    }

    pub fn parse(&mut self) -> Vec<AST> {
        let mut ast = vec![];

        ast.push(self.parse_root());

        ast
    }

    pub fn parse_root(&mut self) -> AST {
        let mut body = vec![];

        body.append(&mut self.parse_statement());

        while let Some(_) = &self.current_token {
            self.eat(TokenKind::NewLine);
            body.append(&mut self.parse_statement());
        }

        let ast = AST::Root { body };

        ast
    }

    pub fn parse_statement(&mut self) -> Vec<AST> {
        if let Some(token) = &self.current_token {
            match TokenKind::from(token) {
                TokenKind::Js => return self.parse_js_literal(),
                TokenKind::Id => {
                    let ast = self.parse_id();
                    let ast = self.parse_binary_expression(ast);
                    return ast;
                }
                _ => {}
            }
        }

        vec![]
    }

    pub fn parse_id(&mut self) -> Vec<AST> {
        let value = self.current_token_value();
        self.eat(TokenKind::Id);

        let result = match value.as_str() {
            "let" => self.parse_variable_definition(),
            "fn" => self.parse_function_definition(),
            _ => {
                let callee = AST::Identifier { value };
                match self.current_token_kind() {
                    TokenKind::LParen => self.parse_function_call(Box::new(callee)),
                    _ => vec![callee],
                }
            }
        };

        return result;
    }

    pub fn parse_function_call(&mut self, callee: Box<AST>) -> Vec<AST> {
        self.eat(TokenKind::LParen);
        self.eat_newline_indefinitely();

        let mut args = self.parse_expression();

        while let Some(_) = &self.current_token {
            match self.current_token_kind() {
                TokenKind::RParen => break,
                TokenKind::NewLine => self.eat_newline_indefinitely(),
                _ => {
                    args.append(&mut self.parse_expression());
                    self.eat_optional(TokenKind::Comma);
                }
            }
        }

        let ast = vec![AST::FunctionCall { callee, args }];
        self.eat_newline_indefinitely();
        self.eat(TokenKind::RParen);
        ast
    }

    pub fn parse_variable_definition(&mut self) -> Vec<AST> {
        self.expect_next(TokenKind::Id);
        let name = self.current_token_value();
        self.eat(TokenKind::Id);
        self.expect_next(TokenKind::Eq);
        self.eat(TokenKind::Eq);
        self.eat_newline_indefinitely();
        let value = self.parse_expression();
        vec![AST::VariableDefinition { name, value }]
    }

    pub fn parse_function_definition(&mut self) -> Vec<AST> {
        let name = self.current_token_value();
        self.eat(TokenKind::Id);

        self.eat(TokenKind::LParen);
        let mut args = vec![];

        if self.current_token_kind() != TokenKind::RParen {
            args.append(&mut self.parse_function_argument_definition());
        }

        while let Some(_) = &self.current_token {
            if self.current_token_kind() == TokenKind::RParen {
                break;
            }
            args.append(&mut self.parse_function_argument_definition());
        }
        self.eat(TokenKind::RParen);

        self.eat(TokenKind::LBrace);
        self.eat_newline_indefinitely();
        let mut body = vec![];
        body.append(&mut self.parse_statement());
        while let Some(Token::NewLine(_, _)) = &self.current_token {
            self.eat(TokenKind::NewLine);
            body.append(&mut self.parse_statement());
        }
        self.eat(TokenKind::RBrace);

        let result = vec![AST::FunctionDefinition { name, args, body }];
        return result;
    }

    pub fn parse_function_argument_definition(&mut self) -> Vec<AST> {
        let name = self.current_token_value();
        self.eat(TokenKind::Id);
        self.eat(TokenKind::DblColon);
        let kind = self.current_token_value();
        self.eat(TokenKind::Id);
        self.eat_optional(TokenKind::Comma);
        let result = vec![AST::FunctionArgumentDefinition { name, kind }];
        return result;
    }

    pub fn parse_expression(&mut self) -> Vec<AST> {
        if let Some(token) = &self.current_token {
            let ast = match TokenKind::from(token) {
                TokenKind::Id => self.parse_id(),
                TokenKind::Str => self.parse_string_literal(),
                TokenKind::Numeric => self.parse_number_literal(),
                TokenKind::LParen => self.parse_parenthesis_expression(),
                _ => vec![],
            };

            let ast = self.parse_binary_expression(ast);

            if ast.len() > 0 {
                return ast;
            }
        }

        vec![]
    }

    pub fn parse_parenthesis_expression(&mut self) -> Vec<AST> {
        self.eat(TokenKind::LParen);
        self.eat_newline_indefinitely();
        let result = self.parse_expression();
        self.eat_newline_until(TokenKind::RParen);
        self.eat(TokenKind::RParen);
        result
    }

    pub fn parse_string_literal(&mut self) -> Vec<AST> {
        let value = self.current_token_value();
        self.eat(TokenKind::Str);
        vec![AST::StringLiteral { value }]
    }

    pub fn parse_number_literal(&mut self) -> Vec<AST> {
        let value = self.current_token_value();
        self.eat(TokenKind::Numeric);
        vec![AST::NumberLiteral { value }]
    }

    pub fn parse_js_literal(&mut self) -> Vec<AST> {
        let value = self.current_token_value();
        self.eat(TokenKind::Js);
        vec![AST::JsLiteral { value }]
    }

    pub fn parse_binary_expression(&mut self, left: Vec<AST>) -> Vec<AST> {
        if let Some(token) = &self.current_token {
            match TokenKind::from(token) {
                TokenKind::Plus => {
                    self.eat(TokenKind::Plus);
                    self.eat_newline_indefinitely();

                    return vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Add,
                        left,
                        right: self.parse_expression(),
                    }];
                }
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus);
                    self.eat_newline_indefinitely();
                    return vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Sub,
                        left,
                        right: self.parse_expression(),
                    }];
                }
                TokenKind::Multiply => {
                    self.eat(TokenKind::Multiply);
                    self.eat_newline_indefinitely();
                    return vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Mul,
                        left,
                        right: self.parse_expression(),
                    }];
                }
                TokenKind::Divide => {
                    self.eat(TokenKind::Divide);
                    self.eat_newline_indefinitely();
                    return vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Div,
                        left,
                        right: self.parse_expression(),
                    }];
                }
                _ => {}
            }
        }

        left
    }

    pub fn eat(&mut self, kind: TokenKind) {
        if let Some(token) = &self.current_token {
            if TokenKind::from(token) == kind {
                self.previous_token = self.current_token.clone();
                self.current_token = self.lexer.get_next_token();
                return;
            }
        }

        self.throw_unexpected_token(&kind)
    }

    pub fn eat_optional(&mut self, kind: TokenKind) {
        if let Some(token) = &self.current_token {
            if TokenKind::from(token) == kind {
                self.previous_token = self.current_token.clone();
                self.current_token = self.lexer.get_next_token();
            }
        }
    }

    pub fn eat_newline_until(&mut self, kind: TokenKind) {
        while let Some(token) = &self.current_token {
            match TokenKind::from(token) {
                tok if tok == kind => {
                    return;
                }
                TokenKind::NewLine => {
                    self.previous_token = self.current_token.clone();
                    self.current_token = self.lexer.get_next_token();
                }
                _ => self.throw_unexpected_token(&kind),
            }
        }

        self.throw_unexpected_token(&kind)
    }

    pub fn eat_newline_indefinitely(&mut self) {
        while let Some(token) = &self.current_token {
            match TokenKind::from(token) {
                tok if tok != TokenKind::NewLine => {
                    return;
                }
                TokenKind::NewLine => {
                    self.previous_token = self.current_token.clone();
                    self.current_token = self.lexer.get_next_token();
                }
                _ => self.throw_unexpected_token(&TokenKind::NewLine),
            }
        }
    }

    pub fn expect_next(&mut self, kind: TokenKind) {
        if let Some(token) = &self.current_token {
            if TokenKind::from(token) == kind {
                self.previous_token = self.current_token.clone();
                return;
            }
        }

        self.throw_unexpected_token(&kind)
    }

    pub fn throw_unexpected_token(&self, kind: &TokenKind) {
        let content = self.lexer.content.clone();
        let source_span = self.current_token_source_span().clone();
        let current_token = self.current_token_or_unknown();

        ErrorMessage::new(
            "".into(),
            content,
            format!(
                "Unexpected token {:?}, expecting token with type {:?}",
                current_token, kind
            ),
            source_span.from,
        )
        .print();

        panic!()
    }

    pub fn current_token_or_unknown(&self) -> Token {
        self.current_token.clone().unwrap_or(Token::unknown())
    }

    pub fn current_token_kind(&self) -> TokenKind {
        TokenKind::from(self.current_token_or_unknown())
    }

    pub fn current_token_value(&self) -> String {
        match self.current_token_or_unknown() {
            Token::Id(v, _) => v,
            Token::Str(v, _) => v,
            Token::Numeric(v, _) => v,
            Token::Plus(v, _) => v,
            Token::Minus(v, _) => v,
            Token::Divide(v, _) => v,
            Token::Multiply(v, _) => v,
            Token::Eq(v, _) => v,
            Token::LParen(v, _) => v,
            Token::RParen(v, _) => v,
            Token::LBrace(v, _) => v,
            Token::RBrace(v, _) => v,
            Token::LargerThan(v, _) => v,
            Token::LessThan(v, _) => v,
            Token::DblColon(v, _) => v,
            Token::Comma(v, _) => v,
            Token::NewLine(v, _) => v,
            Token::Js(v, _) => v,
            Token::Unknown(v, _) => v,
        }
    }

    pub fn current_token_source_span(&self) -> SourceSpan {
        match self.current_token_or_unknown() {
            Token::Id(_, ss) => ss,
            Token::Str(_, ss) => ss,
            Token::Numeric(_, ss) => ss,
            Token::Plus(_, ss) => ss,
            Token::Minus(_, ss) => ss,
            Token::Divide(_, ss) => ss,
            Token::Multiply(_, ss) => ss,
            Token::Eq(_, ss) => ss,
            Token::LParen(_, ss) => ss,
            Token::RParen(_, ss) => ss,
            Token::LBrace(_, ss) => ss,
            Token::RBrace(_, ss) => ss,
            Token::LargerThan(_, ss) => ss,
            Token::LessThan(_, ss) => ss,
            Token::DblColon(_, ss) => ss,
            Token::Comma(_, ss) => ss,
            Token::NewLine(_, ss) => ss,
            Token::Js(_, ss) => ss,
            Token::Unknown(_, ss) => ss,
        }
    }
}

pub fn parse<'a>(lexer: &'a mut Lexer) -> Parser<'a> {
    Parser::new(lexer)
}
