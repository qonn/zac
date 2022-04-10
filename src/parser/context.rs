use crate::error_message::ErrorMessage;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
pub struct ParsingContext<'l> {
    pub lexer: &'l mut Lexer,
    pub prev_token: Token,
    curr_token: Token,
    anon_counter: usize,
}

impl<'l> ParsingContext<'l> {
    pub fn new(lexer: &'l mut Lexer) -> Self {
        let token = lexer.get_next_token(false, false);

        ParsingContext {
            lexer,
            prev_token: token.clone(),
            curr_token: token,
            anon_counter: 1,
        }
    }

    pub fn eat(&mut self, target_kind: TokenKind) {
        if TokenKind::from(self.get_curr_token()) == target_kind {
            let next_token = self.lexer.get_next_token(false, false);
            self.prev_token = self.get_curr_token();
            self.curr_token = next_token;
        } else {
            self.throw_unexpected_token_with_expecting(&target_kind);
        }
    }

    pub fn eat_without_consuming_jsx(&mut self, target_kind: TokenKind) {
        if TokenKind::from(self.get_curr_token()) == target_kind {
            let next_token = self.lexer.get_next_token(true, false);
            self.prev_token = self.get_curr_token();
            self.curr_token = next_token;
        } else {
            self.throw_unexpected_token_with_expecting(&target_kind);
        }
    }

    pub fn eat_jsx(&mut self, target_kind: TokenKind) {
        if TokenKind::from(self.get_curr_token()) == target_kind {
            let next_token = self.lexer.get_next_token(false, true);
            self.prev_token = self.get_curr_token();
            self.curr_token = next_token;
        } else {
            self.throw_unexpected_token_with_expecting(&target_kind);
        }
    }

    pub fn eat_all_newlines(&mut self) {
        match TokenKind::from(self.get_curr_token()) {
            tok if tok != TokenKind::NewLine => {
                return;
            }
            TokenKind::NewLine => {
                let next_token = self.lexer.get_next_token(false, false);
                self.prev_token = self.get_curr_token();
                self.curr_token = next_token;
            }
            _ => self.throw_unexpected_token_with_expecting(&TokenKind::NewLine),
        }
    }

    pub fn eat_all_newlines_jsx(&mut self) {
        match TokenKind::from(self.get_curr_token()) {
            tok if tok != TokenKind::NewLine => {
                return;
            }
            TokenKind::NewLine => {
                let next_token = self.lexer.get_next_token(false, true);
                self.prev_token = self.get_curr_token();
                self.curr_token = next_token;
            }
            _ => self.throw_unexpected_token_with_expecting(&TokenKind::NewLine),
        }
    }

    pub fn peek_ahead_ignoring_newlines(&mut self) -> Token {
        let mut pos = self.lexer.pos;

        loop {
            let (new_pos, token) = self.lexer.peek_ahead(pos, false, false);

            pos = new_pos;

            if token.kind() != TokenKind::NewLine {
                return token;
            }
        }
    }

    pub fn throw_unexpected_token_with_expecting(&mut self, kind: &TokenKind) {
        let filepath = self.lexer.filepath.clone();
        let content = self.lexer.content.clone();
        let current_token = self.get_curr_token();
        let message = format!(
            "Unexpected token {:?}, was expecting token with type {:?}.",
            &current_token.value(),
            kind
        );
        let source_span = current_token.span();

        ErrorMessage::new(filepath, content, message, source_span.from).print();

        panic!()
    }

    pub fn throw_unexpected_token(&mut self) {
        let filepath = self.lexer.filepath.clone();
        let content = self.lexer.content.clone();
        let current_token = self.get_curr_token();
        let message = format!("Unexpected token {:?}.", current_token.value());
        let source_span = current_token.span();

        ErrorMessage::new(filepath, content, message, source_span.from).print();

        panic!()
    }

    pub fn throw_reserved_keywords(&mut self, value: &str) {
        let filepath = self.lexer.filepath.clone();
        let content = self.lexer.content.clone();
        let current_token = self.get_curr_token();
        let message = format!("Reserved keywords {:?} cannot be used here.", value);
        let source_span = current_token.span();

        ErrorMessage::new(filepath, content, message, source_span.from).print();

        panic!()
    }

    pub fn throw_custom(&mut self, message: &str) {
        let filepath = self.lexer.filepath.clone();
        let content = self.lexer.content.clone();
        let current_token = self.get_curr_token();
        let source_span = current_token.span();

        ErrorMessage::new(filepath, content, message.to_string(), source_span.from).print();

        panic!()
    }

    pub fn is_not_eof(&mut self) -> bool {
        if self.curr_token.kind() != TokenKind::Eof {
            true
        } else {
            false
        }
    }

    pub fn get_prev_token(&self) -> Token {
        self.prev_token.clone()
    }

    pub fn get_curr_token(&self) -> Token {
        self.curr_token.clone()
    }

    pub fn get_new_anon_name(&mut self) -> String {
        let v = self.anon_counter;
        self.anon_counter += 1;
        format!("anon_{}", v)
    }
}
