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

        while self.still_has_current_token() {
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
                _ => return self.parse_expression(),
            }
        }

        vec![]
    }

    pub fn parse_id(&mut self) -> Vec<AST> {
        let value = self.current_token_value();

        let span = self.current_token_source_span();

        let result = match value.as_str() {
            "let" => self.parse_variable_definition(),
            "fn" => self.parse_function_definition(),
            "if" => self.parse_if_statement(),
            "type" => self.parse_type_definition(),
            "enum" => self.parse_enum_definition(),
            "rec" => self.parse_record_definition(),
            _ => {
                self.eat(TokenKind::Id);

                let callee = AST::Identifier {
                    value,
                    generics: vec![],
                    span: span.clone(),
                };

                let result = match self.current_token_kind() {
                    TokenKind::LParen => self.parse_function_call(Box::new(callee)),
                    TokenKind::Lt => {
                        let callee = match callee {
                            AST::Identifier {
                                value,
                                generics: _,
                                span,
                            } => AST::Identifier {
                                generics: self.parse_generics_definition(),
                                value,
                                span,
                            },
                            _ => callee,
                        };

                        vec![callee]
                    }
                    _ => vec![callee],
                };

                match self.current_token_kind() {
                    TokenKind::Dot => {
                        let object = Box::from(result[0].clone());
                        self.eat(TokenKind::Dot);
                        let property = Box::from(self.parse_id()[0].clone());
                        let span = SourceSpan::new(span.from, self.previous_token_source_span().to);
                        vec![AST::MemberExpression {
                            object,
                            property,
                            span,
                        }]
                    }
                    _ => result,
                }
            }
        };

        return result;
    }

    pub fn parse_type_definition(&mut self) -> Vec<AST> {
        let span_from = self.current_token_source_span().from;

        self.eat(TokenKind::Id);

        let name = self.current_token_value();

        self.eat(TokenKind::Id);

        // generic definition
        let generics = self.parse_generics_definition();

        let items = if self.current_token_kind() == TokenKind::Eq {
            self.eat(TokenKind::Eq);
            self.parse_expression()
        } else {
            vec![]
        };

        let span_to = self.previous_token_source_span().to;

        let type_definition = AST::TypeDefinition {
            name: name.clone(),
            generics,
            items,
            span: SourceSpan::new(span_from, span_to),
        };

        vec![type_definition]
    }

    pub fn parse_enum_definition(&mut self) -> Vec<AST> {
        let span_from = self.current_token_source_span().from;

        self.eat(TokenKind::Id);

        let name = self.current_token_value();

        self.eat(TokenKind::Id);

        let generics = self.parse_generics_definition();

        self.eat(TokenKind::LBrace);

        let mut items = vec![];

        while self.still_has_current_token() {
            match self.current_token_kind() {
                TokenKind::RBrace => break,
                TokenKind::NewLine => self.eat_newline_indefinitely(),
                _ => {
                    items.append(&mut self.parse_expression());
                    self.eat_newline_indefinitely();
                    if self.current_token_kind() != TokenKind::RBrace {
                        self.eat(TokenKind::Comma);
                    }
                }
            }
        }

        self.eat(TokenKind::RBrace);

        let span_to = self.previous_token_source_span().to;

        let enum_definition = AST::EnumDefinition {
            generics,
            items,
            name: name.clone(),
            span: SourceSpan::new(span_from, span_to),
        };

        vec![enum_definition]
    }

    pub fn parse_record_definition(&mut self) -> Vec<AST> {
        let span_from = self.current_token_source_span().from;

        self.eat(TokenKind::Id);

        let name = self.eat_and_spit_value(TokenKind::Id);

        let mut keys = vec![];

        self.eat(TokenKind::LBrace);

        while self.still_has_current_token() {
            match self.current_token_kind() {
                TokenKind::RBrace => break,
                TokenKind::NewLine => self.eat_newline_indefinitely(),
                _ => {
                    let span_from = self.current_token_source_span().from;
                    let name = self.eat_and_spit_value(TokenKind::Id);
                    self.eat(TokenKind::DblColon);
                    let kind = self.parse_id();
                    self.eat_newline_indefinitely();
                    if self.current_token_kind() != TokenKind::RBrace {
                        self.eat(TokenKind::Comma);
                    }
                    let span_to = self.previous_token_source_span().to;
                    keys.append(&mut vec![AST::RecordKeyDefinition {
                        name,
                        kind,
                        span: SourceSpan::new(span_from, span_to),
                    }])
                }
            }
        }

        self.eat(TokenKind::RBrace);

        let span_to = self.previous_token_source_span().to;

        let record_definition = AST::RecordDefinition {
            name: name.clone(),
            keys,
            span: SourceSpan::new(span_from, span_to),
        };

        vec![record_definition]
    }

    pub fn parse_generics_definition(&mut self) -> Vec<AST> {
        let mut generics = vec![];

        // generic definition
        if self.current_token_kind() == TokenKind::Lt {
            self.eat(TokenKind::Lt);

            while self.still_has_current_token() {
                match self.current_token_kind() {
                    TokenKind::Gt => break,
                    TokenKind::NewLine => self.eat_newline_indefinitely(),
                    _ => {
                        generics.append(&mut self.parse_id());
                        if self.current_token_kind() != TokenKind::Gt {
                            self.eat(TokenKind::Comma);
                        }
                    }
                }
            }

            self.eat(TokenKind::Gt);
        }

        generics
    }

    pub fn parse_function_call(&mut self, callee: Box<AST>) -> Vec<AST> {
        let span_from = callee.source_span().from;

        self.eat(TokenKind::LParen);
        self.eat_newline_indefinitely();

        let mut args = self.parse_expression();

        while self.still_has_current_token() {
            match self.current_token_kind() {
                TokenKind::RParen => break,
                TokenKind::NewLine => self.eat_newline_indefinitely(),
                _ => {
                    args.append(&mut self.parse_expression());
                    self.eat_optional(TokenKind::Comma);
                }
            }
        }

        self.eat_newline_indefinitely();

        self.eat(TokenKind::RParen);

        let span_to = self.previous_token_source_span().to;

        let ast = vec![AST::FunctionCall {
            callee,
            args,
            span: SourceSpan::new(span_from, span_to),
        }];

        ast
    }

    pub fn parse_variable_definition(&mut self) -> Vec<AST> {
        let span_from = self.current_token_source_span().from;
        self.eat(TokenKind::Id);
        self.expect_next(TokenKind::Id);
        let name = self.current_token_value();
        self.eat(TokenKind::Id);
        self.expect_next(TokenKind::Eq);
        self.eat(TokenKind::Eq);
        self.eat_newline_indefinitely();
        let value = self.parse_expression();
        let span_to = self.previous_token_source_span().to;
        let variable_definition = AST::VariableDeclaration {
            name: name.clone(),
            value,
            span: SourceSpan::new(span_from, span_to),
        };
        vec![variable_definition]
    }

    pub fn parse_function_definition(&mut self) -> Vec<AST> {
        let span_from = self.current_token_source_span().from;

        self.eat(TokenKind::Id);

        let name = self.current_token_value();

        self.eat(TokenKind::Id);

        self.eat(TokenKind::LParen);
        let mut args = vec![];

        if self.current_token_kind() != TokenKind::RParen {
            args.append(&mut self.parse_function_argument_definition());
        }

        while self.still_has_current_token() {
            if self.current_token_kind() == TokenKind::RParen {
                break;
            }
            args.append(&mut self.parse_function_argument_definition());
        }

        self.eat(TokenKind::RParen);

        self.eat(TokenKind::LBrace);
        self.eat_newline_indefinitely();

        let body = self.parse_function_definition_body();

        let span_to = self.previous_token_source_span().to;

        let function_definition = AST::FunctionDefinition {
            name: name.clone(),
            args,
            body,
            span: SourceSpan::new(span_from, span_to),
        };

        return vec![function_definition];
    }

    pub fn parse_function_definition_body(&mut self) -> Vec<AST> {
        let mut body = vec![];

        body.append(&mut self.parse_statement());

        while let Some(Token::NewLine(_)) = &self.current_token {
            self.eat(TokenKind::NewLine);
            body.append(&mut self.parse_statement());
        }

        self.eat(TokenKind::RBrace);

        body
    }

    pub fn parse_function_argument_definition(&mut self) -> Vec<AST> {
        let span_from = self.current_token_source_span().from;

        let name = self.current_token_value();
        self.eat(TokenKind::Id);
        self.eat(TokenKind::DblColon);
        let kind = self.parse_id();
        self.eat_optional(TokenKind::Comma);

        let span_to = self.previous_token_source_span().to;

        let result = vec![AST::FunctionArgumentDefinition {
            name,
            kind: Box::new(kind[0].clone()),
            span: SourceSpan::new(span_from, span_to),
        }];
        return result;
    }

    pub fn parse_if_statement(&mut self) -> Vec<AST> {
        let span_from = self.current_token_source_span().from;

        self.eat(TokenKind::Id);

        let test = self.parse_expression();
        let consequence = self.parse_if_statement_consequence();
        let mut alternative = vec![];

        if self.current_token_kind() == TokenKind::Id && self.current_token_value() == "else" {
            self.eat(TokenKind::Id);

            if self.current_token_kind() == TokenKind::Id && self.current_token_value() == "if" {
                alternative.append(&mut self.parse_if_statement());
            } else if self.current_token_kind() == TokenKind::LBrace {
                alternative.append(&mut self.parse_if_statement_consequence());
            }
        }

        let span_to = self.previous_token_source_span().to;

        return vec![AST::IfStatement {
            test,
            consequence,
            alternative,
            span: SourceSpan::new(span_from, span_to),
        }];
    }

    pub fn parse_if_statement_consequence(&mut self) -> Vec<AST> {
        self.eat(TokenKind::LBrace);
        self.eat_newline_indefinitely();
        let mut consequence = vec![];
        consequence.append(&mut self.parse_statement());
        while let Some(Token::NewLine(_)) = &self.current_token {
            self.eat(TokenKind::NewLine);
            consequence.append(&mut self.parse_statement());
        }
        self.eat(TokenKind::RBrace);
        return consequence;
    }

    pub fn parse_expression(&mut self) -> Vec<AST> {
        if let Some(token) = &self.current_token {
            let ast = match TokenKind::from(token) {
                TokenKind::Id => self.parse_id(),
                TokenKind::Str => self.parse_string_literal(),
                TokenKind::Numeric => self.parse_number_literal(),
                TokenKind::LParen => self.parse_parenthesis_expression(),
                TokenKind::LSqrBr => self.parse_array_declarator(),
                _ => vec![],
            };

            let ast = self.parse_binary_expression(ast);

            let ast = match self.current_token_kind() {
                TokenKind::Dot => {
                    let object = Box::from(ast[0].clone());
                    self.eat(TokenKind::Dot);
                    let span_from = self.current_token_source_span().from;
                    let property = Box::from(self.parse_id()[0].clone());
                    let span = SourceSpan::new(span_from, self.previous_token_source_span().to);

                    vec![AST::MemberExpression {
                        object,
                        property,
                        span,
                    }]
                }
                _ => ast,
            };

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

    pub fn parse_array_declarator(&mut self) -> Vec<AST> {
        let span_from = self.current_token_source_span().from;

        self.eat(TokenKind::LSqrBr);
        self.eat_newline_indefinitely();
        let mut items = vec![];

        items.append(&mut self.parse_array_declarator_item());

        while self.still_has_current_token() {
            if self.current_token_kind() == TokenKind::RSqrBr {
                break;
            }
            items.append(&mut self.parse_array_declarator_item());
        }

        self.eat_newline_until(TokenKind::RSqrBr);
        self.eat(TokenKind::RSqrBr);

        let span_to = self.previous_token_source_span().to;

        vec![AST::ArrayDeclarator {
            items,
            span: SourceSpan::new(span_from, span_to),
        }]
    }

    pub fn parse_array_declarator_item(&mut self) -> Vec<AST> {
        let ast = self.parse_expression();
        self.eat_newline_indefinitely();
        if self.current_token_kind() != TokenKind::RSqrBr {
            self.eat(TokenKind::Comma);
        }
        self.eat_newline_indefinitely();
        return ast;
    }

    pub fn parse_string_literal(&mut self) -> Vec<AST> {
        let span = self.current_token_source_span();
        let value = self.current_token_value();
        self.eat(TokenKind::Str);
        vec![AST::StringLiteral { value, span }]
    }

    pub fn parse_number_literal(&mut self) -> Vec<AST> {
        let span = self.current_token_source_span();
        let value = self.current_token_value();
        self.eat(TokenKind::Numeric);
        vec![AST::NumberLiteral { value, span }]
    }

    pub fn parse_js_literal(&mut self) -> Vec<AST> {
        let span = self.current_token_source_span();
        let value = self.current_token_value();
        self.eat(TokenKind::Js);
        vec![AST::JsLiteral { value, span }]
    }

    pub fn parse_binary_expression(&mut self, left: Vec<AST>) -> Vec<AST> {
        let span_from = left
            .iter()
            .map(|x| x.source_span().from)
            .collect::<Vec<_>>()
            .first()
            .unwrap_or(&0)
            .clone();

        let left2 = left.clone();

        if let Some(token) = &self.current_token {
            let result = match TokenKind::from(token) {
                TokenKind::Plus => {
                    self.eat(TokenKind::Plus);
                    self.eat_newline_indefinitely();
                    let right = self.parse_expression();
                    let span_to = self.previous_token_source_span().to;
                    vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Add,
                        left,
                        right,
                        span: SourceSpan::new(span_from, span_to),
                    }]
                }
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus);
                    self.eat_newline_indefinitely();
                    let right = self.parse_expression();
                    let span_to = self.previous_token_source_span().to;
                    vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Sub,
                        left,
                        right,
                        span: SourceSpan::new(span_from, span_to),
                    }]
                }
                TokenKind::Multiply => {
                    self.eat(TokenKind::Multiply);
                    self.eat_newline_indefinitely();
                    let right = self.parse_expression();
                    let span_to = self.previous_token_source_span().to;
                    vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Mul,
                        left,
                        right,
                        span: SourceSpan::new(span_from, span_to),
                    }]
                }
                TokenKind::Divide => {
                    self.eat(TokenKind::Divide);
                    self.eat_newline_indefinitely();
                    let right = self.parse_expression();
                    let span_to = self.previous_token_source_span().to;
                    vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Div,
                        left,
                        right,
                        span: SourceSpan::new(span_from, span_to),
                    }]
                }
                TokenKind::Lt => {
                    self.eat(TokenKind::Lt);
                    self.eat_newline_indefinitely();
                    let right = self.parse_expression();
                    let span_to = self.previous_token_source_span().to;
                    vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Lt,
                        left,
                        right,
                        span: SourceSpan::new(span_from, span_to),
                    }]
                }
                TokenKind::Gt => {
                    self.eat(TokenKind::Gt);
                    self.eat_newline_indefinitely();
                    let right = self.parse_expression();
                    let span_to = self.previous_token_source_span().to;
                    vec![AST::BinaryExpression {
                        kind: ASTBinaryExpressionKind::Gt,
                        left,
                        right,
                        span: SourceSpan::new(span_from, span_to),
                    }]
                }
                _ => vec![],
            };

            if result.len() > 0 {
                return result;
            }
        }

        left2
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

    pub fn eat_and_spit_value(&mut self, kind: TokenKind) -> String {
        let value = self.current_token_value();
        self.eat(kind);
        value
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
        self.current_token
            .clone()
            .unwrap_or(Token::Eof(SourceSpan::new(
                self.lexer.len - 1,
                self.lexer.len - 1,
            )))
    }

    pub fn previous_token_or_unknown(&self) -> Token {
        self.previous_token
            .clone()
            .unwrap_or(Token::Eof(SourceSpan::new(
                self.lexer.len - 1,
                self.lexer.len - 1,
            )))
    }

    pub fn current_token_kind(&self) -> TokenKind {
        TokenKind::from(self.current_token_or_unknown())
    }

    pub fn current_token_value(&self) -> String {
        match self.current_token_or_unknown() {
            Token::Id(v, _) => v,
            Token::Str(v, _) => v,
            Token::Numeric(v, _) => v,
            Token::Plus(_) => String::from("+"),
            Token::Minus(_) => String::from("-"),
            Token::Divide(_) => String::from("/"),
            Token::Multiply(_) => String::from("*"),
            Token::Eq(_) => String::from("="),
            Token::LParen(_) => String::from("("),
            Token::RParen(_) => String::from(")"),
            Token::LBrace(_) => String::from("{"),
            Token::RBrace(_) => String::from("}"),
            Token::Gt(_) => String::from(">"),
            Token::Lt(_) => String::from("<"),
            Token::DblColon(_) => String::from(":"),
            Token::Comma(_) => String::from(","),
            Token::NewLine(_) => String::from("\n"),
            Token::Js(v, _) => v,
            Token::JsxOpen(v, _) => v,
            Token::Eof(_) => String::from(""),
            Token::LSqrBr(_) => String::from("["),
            Token::RSqrBr(_) => String::from("]"),
            Token::Dot(_) => String::from("."),
        }
    }

    pub fn current_token_source_span(&self) -> SourceSpan {
        match self.current_token_or_unknown() {
            Token::Id(_, ss) => ss,
            Token::Str(_, ss) => ss,
            Token::Numeric(_, ss) => ss,
            Token::Js(_, ss) => ss,
            Token::JsxOpen(_, ss) => ss,
            Token::Plus(ss) => ss,
            Token::Minus(ss) => ss,
            Token::Divide(ss) => ss,
            Token::Multiply(ss) => ss,
            Token::Eq(ss) => ss,
            Token::LParen(ss) => ss,
            Token::RParen(ss) => ss,
            Token::LBrace(ss) => ss,
            Token::RBrace(ss) => ss,
            Token::Gt(ss) => ss,
            Token::Lt(ss) => ss,
            Token::DblColon(ss) => ss,
            Token::Comma(ss) => ss,
            Token::NewLine(ss) => ss,
            Token::Eof(ss) => ss,
            Token::LSqrBr(ss) => ss,
            Token::RSqrBr(ss) => ss,
            Token::Dot(ss) => ss,
        }
    }

    pub fn previous_token_source_span(&self) -> SourceSpan {
        match self.previous_token_or_unknown() {
            Token::Id(_, ss) => ss,
            Token::Str(_, ss) => ss,
            Token::Numeric(_, ss) => ss,
            Token::Js(_, ss) => ss,
            Token::JsxOpen(_, ss) => ss,
            Token::Plus(ss) => ss,
            Token::Minus(ss) => ss,
            Token::Divide(ss) => ss,
            Token::Multiply(ss) => ss,
            Token::Eq(ss) => ss,
            Token::LParen(ss) => ss,
            Token::RParen(ss) => ss,
            Token::LBrace(ss) => ss,
            Token::RBrace(ss) => ss,
            Token::Gt(ss) => ss,
            Token::Lt(ss) => ss,
            Token::DblColon(ss) => ss,
            Token::Comma(ss) => ss,
            Token::NewLine(ss) => ss,
            Token::Eof(ss) => ss,
            Token::LSqrBr(ss) => ss,
            Token::RSqrBr(ss) => ss,
            Token::Dot(ss) => ss,
        }
    }

    pub fn still_has_current_token(&self) -> bool {
        if let Some(_) = &self.current_token {
            true
        } else {
            false
        }
    }
}

pub fn parse<'a>(lexer: &'a mut Lexer) -> Parser<'a> {
    Parser::new(lexer)
}
