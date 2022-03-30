use std::collections::HashMap;

use crate::{ast::AST, token::SourceSpan};

#[derive(Debug, Clone)]
pub struct Scope {
    pub type_definitions: HashMap<String, AST>,
    pub enum_definitions: HashMap<String, AST>,
    pub records_definitions: HashMap<String, AST>,
    pub variable_definitions: HashMap<String, AST>,
    pub function_definitions: HashMap<String, AST>,
}

impl Scope {
    pub fn add_type_definition(&mut self, identifier: &String, ast: &AST) -> &mut Self {
        self.type_definitions
            .insert(identifier.clone(), ast.clone());
        self
    }

    pub fn get_type_definition(&self, identifier: &String) -> Option<&AST> {
        self.type_definitions.get(identifier)
    }

    pub fn add_enum_definition(&mut self, identifier: &String, ast: &AST) -> &mut Self {
        self.enum_definitions
            .insert(identifier.clone(), ast.clone());
        self
    }

    pub fn get_enum_definition(&self, identifier: &String) -> Option<&AST> {
        self.enum_definitions.get(identifier)
    }

    pub fn add_record_definition(&mut self, identifier: &String, ast: &AST) -> &mut Self {
        self.records_definitions
            .insert(identifier.clone(), ast.clone());
        self
    }

    pub fn get_record_definition(&self, identifier: &String) -> Option<&AST> {
        self.records_definitions.get(identifier)
    }

    pub fn add_variable_definition(&mut self, identifier: &String, ast: &AST) -> &mut Self {
        self.variable_definitions
            .insert(identifier.clone(), ast.clone());
        self
    }

    pub fn get_variable_definition(&self, identifier: &String) -> Option<&AST> {
        self.variable_definitions.get(identifier)
    }

    pub fn add_function_definition(&mut self, identifier: &String, ast: &AST) -> &mut Self {
        self.function_definitions
            .insert(identifier.clone(), ast.clone());
        self
    }

    pub fn get_function_definition(&self, identifier: &String) -> Option<&AST> {
        self.function_definitions.get(identifier)
    }

    pub fn is_defined(&self, identifier: &String) -> bool {
        if let Some(_) = self.get_enum_definition(identifier) {
            true
        } else if let Some(_) = self.get_function_definition(identifier) {
            true
        } else if let Some(_) = self.get_record_definition(identifier) {
            true
        } else if let Some(_) = self.get_type_definition(identifier) {
            true
        } else if let Some(_) = self.get_variable_definition(identifier) {
            true
        } else {
            false
        }
    }

    pub fn find_definition(&self, identifier: &String) -> Option<&AST> {
        if let Some(v) = self.get_enum_definition(identifier) {
            Some(v)
        } else if let Some(v) = self.get_function_definition(identifier) {
            Some(v)
        } else if let Some(v) = self.get_record_definition(identifier) {
            Some(v)
        } else if let Some(v) = self.get_type_definition(identifier) {
            Some(v)
        } else if let Some(v) = self.get_variable_definition(identifier) {
            Some(v)
        } else {
            None
        }
    }

    pub fn clear_definition_for(&mut self, identifier: &String) {
        self.enum_definitions.remove(identifier);
        self.function_definitions.remove(identifier);
        self.records_definitions.remove(identifier);
        self.type_definitions.remove(identifier);
        self.variable_definitions.remove(identifier);
    }
}

pub fn new() -> Scope {
    let mut type_definitions: HashMap<String, _> = HashMap::new();

    type_definitions.insert(
        "Unit".into(),
        AST::BuiltinReservation {
            span: SourceSpan::new(0, 0),
        },
    );

    type_definitions.insert(
        "Boolean".into(),
        AST::BuiltinReservation {
            span: SourceSpan::new(0, 0),
        },
    );

    type_definitions.insert(
        "String".into(),
        AST::BuiltinReservation {
            span: SourceSpan::new(0, 0),
        },
    );
    type_definitions.insert(
        "Number".into(),
        AST::BuiltinReservation {
            span: SourceSpan::new(0, 0),
        },
    );
    type_definitions.insert(
        "Vec".into(),
        AST::BuiltinReservation {
            span: SourceSpan::new(0, 0),
        },
    );

    Scope {
        type_definitions,
        function_definitions: HashMap::new(),
        variable_definitions: HashMap::new(),
        enum_definitions: HashMap::new(),
        records_definitions: HashMap::new(),
    }
}
