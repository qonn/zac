use std::{collections::HashMap, hash::Hash};

use crate::{error_message::ErrorMessage, scope::Scope};

pub struct CheckerContext {
    pub filepath: String,
    pub content: String,
    pub scopes: HashMap<String, Scope>,
}

impl CheckerContext {
    pub fn print_error_message(&self, message: String, pos: usize) {
        let filepath = self.filepath.clone();
        let context = self.content.clone();
        ErrorMessage::new(filepath, context, message, pos).print();
    }

    pub fn add_scope(&mut self, identifier: &String, scope: Scope) {
        self.scopes.insert(identifier.clone(), scope);
    }

    pub fn get_scope(&self, identifier: &String) -> Option<&Scope> {
        self.scopes.get(identifier)
    }
}

pub fn new(filename: &String, content: &String) -> CheckerContext {
    CheckerContext {
        filepath: filename.clone(),
        content: content.clone(),
        scopes: HashMap::new(),
    }
}
