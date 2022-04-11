use crate::{error_message::ErrorMessage, span::Span, token::Token};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref COMMENT_LINE: Regex = Regex::new(r"^(//.+)").unwrap();
    static ref COMMENT_BLOCK: Regex = Regex::new(r"^(/\*(?s)(.*?)\*/)").unwrap();
    static ref STRING_LITERAL: Regex = Regex::new(r#"^\s*"([^"\\]*(?s:\\.[^"\\]*)*)""#).unwrap();
    static ref NUMBER_LITERAL: Regex = Regex::new(r#"^([+-]?([0-9]*[.])?[0-9]+)"#).unwrap();
    static ref BOOLEAN_LITERAL: Regex = Regex::new(r#"^((true)|(false))"#).unwrap();
    static ref JS_LITERAL: Regex = Regex::new(r"^`(?s)(.*?)`").unwrap();
    static ref JSX_A: Regex = Regex::new(r"^<([A-Za-z0-9]+)(?s:.*?)>").unwrap();
    static ref JSX_STRING_LITERAL: Regex = Regex::new(r#"^[^<>\{\}\r\n]+"#).unwrap();
    static ref JSX_SELF_CLOSE: Regex = Regex::new(r"^/>").unwrap();
    static ref JSX_CLOSE: Regex = Regex::new(r"^</([a-zA-Z0-9]+)>").unwrap();
    static ref IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z0-9_]+").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"^[ \r\t]+").unwrap();
    static ref DOT: Regex = Regex::new(r"^\.").unwrap();
    static ref COLON: Regex = Regex::new(r"^:").unwrap();
    static ref COMMA: Regex = Regex::new(r"^,").unwrap();
    static ref LT: Regex = Regex::new(r"^<").unwrap();
    static ref GT: Regex = Regex::new(r"^>").unwrap();
    static ref LPAREN: Regex = Regex::new(r"^\(").unwrap();
    static ref RPAREN: Regex = Regex::new(r"^\)").unwrap();
    static ref LBRACE: Regex = Regex::new(r"^\{").unwrap();
    static ref RBRACE: Regex = Regex::new(r"^\}").unwrap();
    static ref LBRCKT: Regex = Regex::new(r"^\[").unwrap();
    static ref RBRCKT: Regex = Regex::new(r"^\]").unwrap();
    static ref ADD: Regex = Regex::new(r"^+").unwrap();
    static ref SUB: Regex = Regex::new(r"^-").unwrap();
    static ref DIV: Regex = Regex::new(r"^/").unwrap();
    static ref MUL: Regex = Regex::new(r"^/").unwrap();
    static ref NE: Regex = Regex::new(r"^!=").unwrap();
    static ref EQ: Regex = Regex::new(r"^==").unwrap();
    static ref ASSIGNMENT: Regex = Regex::new(r"^=").unwrap();
    static ref NEWLINE: Regex = Regex::new(r"^[\n]+").unwrap();
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub filepath: String,
    pub content: String,
    pub len: usize,
    pub pos: usize,
    pub lines: usize,
}

impl Lexer {
    pub fn new(filepath: String, content: String) -> Self {
        let len = content.len();

        Lexer {
            filepath,
            content,
            len,
            pos: 0,
            lines: 0,
        }
    }

    pub fn get_next_token(&mut self, do_not_consume_jsx: bool, inside_jsx: bool) -> Token {
        let result = self.get_next_token_ex(do_not_consume_jsx, inside_jsx);
        // println!("{:?}", result);
        return result;
    }

    pub fn get_next_token_ex(&mut self, do_not_consume_jsx: bool, inside_jsx: bool) -> Token {
        let (next_pos, token) = self.peek_ahead(self.pos, do_not_consume_jsx, inside_jsx);
        self.pos = next_pos;
        token
    }

    pub fn peek_ahead(
        &mut self,
        starting_pos: usize,
        do_not_consume_jsx: bool,
        inside_jsx: bool,
    ) -> (usize, Token) {
        let mut current_pos = starting_pos;

        loop {
            if current_pos == self.len {
                return (self.len, Token::Eof(Span::new(self.len, self.len)));
            }

            let slice = &self.content.clone()[current_pos..];

            match slice {
                _ if COMMENT_LINE.is_match(slice) => {
                    let cap = &COMMENT_LINE.captures(slice).unwrap()[1];
                    current_pos = self.peek_ahead_advance(current_pos, cap.len());
                }
                _ if COMMENT_BLOCK.is_match(slice) => {
                    let cap = &COMMENT_BLOCK.captures(slice).unwrap()[1];
                    current_pos = self.peek_ahead_advance(current_pos, cap.len());
                }
                _ if WHITESPACE.is_match(slice) => {
                    let cap = &WHITESPACE.captures(slice).unwrap()[0];
                    current_pos = self.peek_ahead_advance(current_pos, cap.len());
                }
                _ if JS_LITERAL.is_match(slice) => {
                    let caps = JS_LITERAL.captures(slice).unwrap();
                    let cap_overall = &caps[0];
                    let cap_inner = &caps[1];
                    let token = Token::Js(
                        cap_inner.to_string(),
                        self.span(current_pos, cap_overall.len()),
                    );
                    return self.peek_ahead_advance_with_token(
                        current_pos,
                        cap_overall.len(),
                        token,
                    );
                }
                _ if JSX_SELF_CLOSE.is_match(slice) => {
                    let caps = JSX_SELF_CLOSE.captures(slice).unwrap();
                    let cap = &caps[0];
                    let token = Token::JsxSelfClose(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if JSX_CLOSE.is_match(slice) => {
                    let caps = JSX_CLOSE.captures(slice).unwrap();
                    let cap = &caps[0];
                    let name = &caps[1];
                    let token =
                        Token::JsxClose(name.to_string(), self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if inside_jsx && JSX_STRING_LITERAL.is_match(slice) => {
                    let caps = JSX_STRING_LITERAL.captures(slice).unwrap();
                    let cap = &caps[0];
                    let token = Token::Str(cap.to_string(), self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if !inside_jsx && STRING_LITERAL.is_match(slice) => {
                    let caps = STRING_LITERAL.captures(slice).unwrap();
                    let cap1 = &caps[0];
                    let cap2 = &caps[1];
                    let token = Token::Str(cap2.to_string(), self.span(current_pos, cap1.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap1.len(), token);
                }
                _ if NUMBER_LITERAL.is_match(slice) => {
                    let caps = NUMBER_LITERAL.captures(slice).unwrap();
                    let cap1 = &caps[0];
                    let token =
                        Token::Numeric(cap1.to_string(), self.span(current_pos, cap1.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap1.len(), token);
                }
                _ if BOOLEAN_LITERAL.is_match(slice) => {
                    let caps = BOOLEAN_LITERAL.captures(slice).unwrap();
                    let cap1 = &caps[0];
                    let token =
                        Token::Boolean(cap1.to_string(), self.span(current_pos, cap1.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap1.len(), token);
                }
                _ if IDENTIFIER.is_match(slice) => {
                    let cap = &IDENTIFIER.captures(slice).unwrap()[0];
                    let span = self.span(current_pos, cap.len());
                    let token = match cap {
                        "fn" => Token::Fn(span),
                        "return" => Token::Return(span),
                        "mod" => Token::Mod(span),
                        "let" => Token::Let(span),
                        _ => Token::Id(cap.to_string(), span),
                    };
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if NEWLINE.is_match(slice) => {
                    let cap = &NEWLINE.captures(slice).unwrap()[0];
                    let token = Token::NewLine(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if DOT.is_match(slice) => {
                    let cap = &DOT.captures(slice).unwrap()[0];
                    let token = Token::Dot(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if COLON.is_match(slice) => {
                    let cap = &COLON.captures(slice).unwrap()[0];
                    let token = Token::DblColon(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if COMMA.is_match(slice) => {
                    let cap = &COMMA.captures(slice).unwrap()[0];
                    let token = Token::Comma(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if GT.is_match(slice) => {
                    let cap = &GT.captures(slice).unwrap()[0];
                    let token = Token::Gt(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if LT.is_match(slice) => {
                    if !do_not_consume_jsx && JSX_A.is_match(slice) {
                        let caps = JSX_A.captures(slice).unwrap();
                        let token = Token::JsxOpen(
                            caps[1].to_string(),
                            self.span(current_pos, caps[1].len()),
                        );
                        return self.peek_ahead_advance_with_token(
                            current_pos,
                            caps[1].len() + 1,
                            token,
                        );
                    }

                    let cap = &LT.captures(slice).unwrap()[0];
                    let token = Token::Lt(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if LPAREN.is_match(slice) => {
                    let cap = &LPAREN.captures(slice).unwrap()[0];
                    let token = Token::LParen(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if RPAREN.is_match(slice) => {
                    let cap = &RPAREN.captures(slice).unwrap()[0];
                    let token = Token::RParen(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if LBRACE.is_match(slice) => {
                    let cap = &LBRACE.captures(slice).unwrap()[0];
                    let token = Token::LBrace(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if RBRACE.is_match(slice) => {
                    let cap = &RBRACE.captures(slice).unwrap()[0];
                    let token = Token::RBrace(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if LBRCKT.is_match(slice) => {
                    let cap = &LBRCKT.captures(slice).unwrap()[0];
                    let token = Token::LSqrBr(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if RBRCKT.is_match(slice) => {
                    let cap = &RBRCKT.captures(slice).unwrap()[0];
                    let token = Token::RSqrBr(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ if ASSIGNMENT.is_match(slice) => {
                    let cap = &ASSIGNMENT.captures(slice).unwrap()[0];
                    let token = Token::Eq(self.span(current_pos, cap.len()));
                    return self.peek_ahead_advance_with_token(current_pos, cap.len(), token);
                }
                _ => {
                    let character = &self.content[current_pos..current_pos + 1];

                    ErrorMessage::new(
                        "".into(),
                        self.content.clone(),
                        format!(
                            "I'm sorry but we do not support the character '{}'",
                            match character {
                                "\r" => "\\r",
                                "\t" => "\\t",
                                "\n" => "\\n",
                                " " => "{space}",
                                _ => character,
                            }
                            .to_string()
                        ),
                        current_pos,
                    )
                    .print();
                    panic!()
                }
            }
        }
    }

    pub fn peek_ahead_advance(&mut self, current_pos: usize, how_many: usize) -> usize {
        current_pos + how_many
    }

    pub fn peek_ahead_advance_with_token(
        &mut self,
        current_pos: usize,
        how_many: usize,
        token: Token,
    ) -> (usize, Token) {
        (self.peek_ahead_advance(current_pos, how_many), token)
    }

    pub fn advance(&mut self, how_many: usize) -> &mut Self {
        self.pos += how_many;
        self
    }

    pub fn advance_with_token(&mut self, how_many: usize, token: Token) -> Token {
        self.advance(how_many);
        token
    }

    pub fn span(&mut self, current_pos: usize, how_many: usize) -> Span {
        Span::new(current_pos, current_pos + how_many)
    }
}

pub(crate) fn new(filepath: &String, content: &String) -> Lexer {
    Lexer::new(filepath.clone(), content.clone())
}
