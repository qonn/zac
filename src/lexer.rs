use crate::{
    error_message::ErrorMessage,
    token::{SourceSpan, Token},
};

#[derive(Debug, Clone)]
pub struct Lexer {
    content: String,
    tokens: Vec<Token>,
    len: usize,
    pos: usize,
    lines: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        let len = content.len();

        Lexer {
            content,
            len,
            tokens: vec![],
            pos: 0,
            lines: 0,
        }
    }

    pub fn lex(mut self) -> Self {
        while let Some(token) = self.get_next_token() {
            self.tokens.push(token);
        }

        self
    }

    pub fn char(&self) -> Option<char> {
        if self.pos == self.len {
            return None;
        }

        self.content[self.pos..self.pos + 1].chars().nth(0)
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.char() {
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
                _ if self.char_is_numeric() => {
                    return Some(self.collect_numeric());
                }
                '=' => {
                    return Some(self.advance_with_token(Token::Eq {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                '+' => {
                    return Some(self.advance_with_token(Token::Plus {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                '-' => {
                    return Some(self.advance_with_token(Token::Minus {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                '/' => {
                    return Some(self.advance_with_token(Token::Divide {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                '*' => {
                    return Some(self.advance_with_token(Token::Multiply {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                '{' => {
                    return Some(self.advance_with_token(Token::LBrace {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                '}' => {
                    return Some(self.advance_with_token(Token::RBrace {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                '(' => {
                    return Some(self.advance_with_token(Token::LParen {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                ')' => {
                    return Some(self.advance_with_token(Token::RParen {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                ':' => {
                    return Some(self.advance_with_token(Token::DblColon {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                ',' => {
                    return Some(self.advance_with_token(Token::Comma {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }))
                }
                ' ' | '\t' | '\r' => {
                    self.skip_whitespace();
                }
                '\n' => {
                    self.lines += 1;
                    return Some(self.advance_with_token(Token::NewLine {
                        span: SourceSpan::new(self.lines, self.pos, self.pos + 1),
                    }));
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

    pub fn char_is_numeric(&self) -> bool {
        if let Some(c) = self.char() {
            (c >= '0' && c <= '9') || c == '.'
        } else {
            false
        }
    }

    pub fn skip_whitespace(&mut self) -> &mut Self {
        while self.char_is_whitespace() {
            self.advance();
        }

        self
    }

    fn collect_id(&mut self) -> Token {
        let start_pos = self.pos;

        while self.char_is_word() {
            self.advance();
        }

        let end_pos = self.pos;

        Token::Id {
            span: SourceSpan::new(self.lines, start_pos, end_pos),
            content: self.content[start_pos..end_pos].to_string(),
        }
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

        Token::Str {
            span: SourceSpan::new(self.lines, start_pos, end_pos),
            content: collected_string,
        }
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

        Token::Js {
            span: SourceSpan::new(self.lines, start_pos, end_pos),
            content: collected_string,
        }
    }

    fn collect_numeric(&mut self) -> Token {
        let start_pos = self.pos;

        while self.char_is_numeric() {
            self.advance();
        }

        let end_pos = self.pos;

        let collected_string = self.content[start_pos..end_pos].to_string();

        Token::Numeric {
            span: SourceSpan::new(self.lines, start_pos, end_pos),
            content: collected_string,
        }
    }
}

pub(crate) fn lex(content: String) -> Vec<Token> {
    Lexer::new(content).lex().tokens.clone()
}
