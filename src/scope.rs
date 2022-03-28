use std::collections::HashMap;

use crate::ast::AST;

pub struct Scope {
    pub type_definitions: HashMap<String, AST>,
    pub variable_definitions: HashMap<String, AST>,
    pub function_definitions: HashMap<String, AST>,
}

impl Scope {
    pub fn add_type_definition(&mut self, identifier: &String, ast: AST) -> &mut Self {
        self.type_definitions.insert(identifier.clone(), ast);
        self
    }

    pub fn get_type_definition(&self, identifier: &String) -> Option<&AST> {
        self.type_definitions.get(identifier)
    }

    pub fn add_variable_definition(&mut self, identifier: &String, ast: AST) -> &mut Self {
        self.variable_definitions.insert(identifier.clone(), ast);
        self
    }

    pub fn get_variable_definition(&self, identifier: &String) -> Option<&AST> {
        self.variable_definitions.get(identifier)
    }

    pub fn add_function_definition(&mut self, identifier: &String, ast: AST) -> &mut Self {
        self.function_definitions.insert(identifier.clone(), ast);
        self
    }

    pub fn get_function_definition(&self, identifier: &String) -> Option<&AST> {
        self.function_definitions.get(identifier)
    }
}

pub fn new() -> Scope {
    Scope {
        type_definitions: HashMap::new(),
        function_definitions: HashMap::new(),
        variable_definitions: HashMap::new(),
    }
}
