use std::collections::HashMap;

use crate::{ast::AST, error_message::ErrorMessage, scope::Scope};

pub struct CheckingContext {
    pub filepath: String,
    pub content: String,
    pub scopes: HashMap<String, Box<Scope>>,
}

impl CheckingContext {
    pub fn print_error_message(&self, message: String, pos: usize) {
        let filepath = self.filepath.clone();
        let context = self.content.clone();
        ErrorMessage::new(filepath, context, message, pos).print();
    }

    pub fn add_scope(&mut self, identifier: &String, scope: Scope) {
        self.scopes.insert(identifier.clone(), Box::new(scope));
    }

    pub fn get_scope(&self, identifier: &String) -> Option<&Box<Scope>> {
        self.scopes.get(identifier)
    }
}

pub fn new(filename: &String, content: &String) -> CheckingContext {
    CheckingContext {
        filepath: filename.clone(),
        content: content.clone(),
        scopes: HashMap::new(),
    }
}
