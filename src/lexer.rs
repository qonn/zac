use crate::{
    error_message::ErrorMessage,
    token::{SourceSpan, Token},
};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub content: String,
    pub len: usize,
    pub pos: usize,
    pub lines: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        let len = content.len();

        Lexer {
            content,
            len,
            pos: 0,
            lines: 0,
        }
    }

    pub fn char(&self) -> Option<char> {
        if self.pos == self.len {
            return None;
        }

        self.content[self.pos..self.pos + 1].chars().nth(0)
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.char() {
            let span = self.span();

            match c {
                _ if self.char_is_word() => {
                    return Some(self.collect_id());
                }
                _ if self.char_is_quote() => {
                    return Some(self.collect_string());
                }
                _ if self.char_is_tilde() => {
                    return Some(self.collect_js());
                }
                _ if self.char_is_numeric(false) => {
                    return Some(self.collect_numeric());
                }
                '=' => return Some(self.advance_with_token(Token::Eq(span))),
                '+' => return Some(self.advance_with_token(Token::Plus(span))),
                '-' => return Some(self.advance_with_token(Token::Minus(span))),
                '/' => {
                    self.advance();

                    if self.char().unwrap_or(' ') == '/' {
                        self.skip_line();
                        continue;
                    } else if self.char().unwrap_or(' ') == '*' {
                        self.skip_comment_block();
                        continue;
                    }

                    return Some(Token::Divide(span));
                }
                '*' => return Some(self.advance_with_token(Token::Multiply(span))),

                '.' => return Some(self.advance_with_token(Token::Dot(span))),
                '{' => return Some(self.advance_with_token(Token::LBrace(span))),
                '}' => return Some(self.advance_with_token(Token::RBrace(span))),
                '(' => return Some(self.advance_with_token(Token::LParen(span))),
                ')' => return Some(self.advance_with_token(Token::RParen(span))),
                '[' => return Some(self.advance_with_token(Token::LSqrBr(span))),
                ']' => return Some(self.advance_with_token(Token::RSqrBr(span))),
                ':' => return Some(self.advance_with_token(Token::DblColon(span))),
                ',' => return Some(self.advance_with_token(Token::Comma(span))),
                '>' => return Some(self.advance_with_token(Token::Gt(span))),
                '<' => return Some(self.advance_with_token(Token::Lt(span))),
                '\n' => {
                    self.lines += 1;
                    return Some(self.advance_with_token(Token::NewLine(span)));
                }
                ' ' | '\t' | '\r' => {
                    self.skip_whitespace();
                }
                _ => {
                    ErrorMessage::new(
                        "".into(),
                        self.content.clone(),
                        format!("I'm sorry but we do not support the character '{}'", c),
                        self.pos,
                    )
                    .print();
                    panic!()
                }
            }
        }

        None
    }

    pub fn advance(&mut self) -> &mut Self {
        self.pos += 1;
        self
    }

    pub fn span(&mut self) -> SourceSpan {
        SourceSpan::new(self.pos, self.pos + 1)
    }

    pub fn advance_with_token(&mut self, token: Token) -> Token {
        self.advance();
        token
    }

    pub fn char_is_whitespace(&self) -> bool {
        if let Some(c) = self.char() {
            c.eq(&' ') || c.eq(&'\t') || c.eq(&'\r')
        } else {
            false
        }
    }

    pub fn char_is_quote(&self) -> bool {
        if let Some(c) = self.char() {
            c == '\"'
        } else {
            false
        }
    }

    pub fn char_is_tilde(&self) -> bool {
        if let Some(c) = self.char() {
            c.eq(&'`')
        } else {
            false
        }
    }

    pub fn char_is_word(&self) -> bool {
        if let Some(c) = self.char() {
            (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
        } else {
            false
        }
    }

    pub fn char_is_word_and_numeric(&self) -> bool {
        if let Some(c) = self.char() {
            (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_'
        } else {
            false
        }
    }

    pub fn char_is_numeric(&self, include_dot: bool) -> bool {
        if let Some(c) = self.char() {
            (c >= '0' && c <= '9') || if include_dot { c == '.' } else { false }
        } else {
            false
        }
    }

    pub fn char_is_newline_or_eof(&mut self) -> bool {
        if let Some(c) = self.char() {
            c.eq(&'\n')
        } else {
            true
        }
    }

    pub fn skip_whitespace(&mut self) -> &mut Self {
        while self.char_is_whitespace() {
            self.advance();
        }

        self
    }

    pub fn skip_line(&mut self) -> &mut Self {
        while !self.char_is_newline_or_eof() {
            self.advance();
        }

        self
    }

    pub fn skip_comment_block(&mut self) -> &mut Self {
        while let Some(c) = self.char() {
            if c.eq(&'*') {
                self.advance();
                if self.char().unwrap_or(' ') == '/' {
                    self.advance();
                    return self;
                }
            }

            self.advance();
        }

        self
    }

    fn collect_id(&mut self) -> Token {
        let start_pos = self.pos;

        while self.char_is_word_and_numeric() {
            self.advance();
        }

        let end_pos = self.pos;

        Token::Id(
            self.content[start_pos..end_pos].to_string(),
            SourceSpan::new(start_pos, end_pos),
        )
    }

    fn collect_string(&mut self) -> Token {
        self.advance();

        let start_pos = self.pos;

        while !self.char_is_quote() {
            self.advance();
        }

        let end_pos = self.pos;

        let collected_string = self.content[start_pos..end_pos].to_string();

        self.advance();

        Token::Str(collected_string, SourceSpan::new(start_pos, end_pos))
    }

    fn collect_js(&mut self) -> Token {
        self.advance();

        let start_pos = self.pos;

        while !self.char_is_tilde() {
            self.advance();
        }

        let end_pos = self.pos;

        let collected_string = self.content[start_pos..end_pos].to_string();

        self.advance();

        Token::Js(collected_string, SourceSpan::new(start_pos, end_pos))
    }

    fn collect_numeric(&mut self) -> Token {
        let start_pos = self.pos;

        while self.char_is_numeric(true) {
            self.advance();
        }

        let end_pos = self.pos;

        let collected_string = self.content[start_pos..end_pos].to_string();

        Token::Numeric(collected_string, SourceSpan::new(start_pos, end_pos))
    }
}

pub(crate) fn new(content: &String) -> Lexer {
    Lexer::new(content.clone())
}
