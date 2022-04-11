use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref LOWERCASED: Regex = Regex::new(r"([a-z]+)").unwrap();
}
use crate::{
    ast::{self, Type},
    error_message::ErrorMessage,
    span::{Span, Spanned},
};

#[derive(Debug, Clone)]
pub struct InferedType {
    pub id: String,
    pub generics: Vec<String>,
}

impl From<ast::Type> for InferedType {
    fn from(t: ast::Type) -> Self {
        match t {
            ast::Type::Default => InferedType {
                id: "Unit".into(),
                generics: vec![],
            },
            ast::Type::Ident(id) => InferedType {
                id: id.string.to_string(),
                generics: id
                    .generics
                    .iter()
                    .map(|generic| generic.string.to_string())
                    .collect::<Vec<_>>(),
            },
        }
    }
}

impl Into<ast::Ident> for InferedType {
    fn into(self) -> ast::Ident {
        ast::Ident {
            string: self.id.clone(),
            generics: self
                .generics
                .iter()
                .map(|g| ast::Ident {
                    string: g.clone(),
                    generics: vec![],
                    span: Span::empty(),
                })
                .collect::<Vec<_>>(),
            span: Span::empty(),
        }
    }
}

#[derive(Clone)]
pub struct Context {
    pub module_path: String,
    pub file_path: String,
    pub file_content: String,
    pub mod_defs: HashMap<String, ast::Mod>,
    pub fn_defs: HashMap<String, ast::Fn>,
    pub var_defs: HashMap<String, ast::Let>,
    pub resolved_type_defs: HashMap<String, InferedType>,
}

impl Context {
    pub fn new(file_path: String, file_content: String) -> Context {
        Context {
            file_path,
            file_content,
            module_path: "".into(),
            fn_defs: HashMap::new(),
            mod_defs: HashMap::new(),
            var_defs: HashMap::new(),
            resolved_type_defs: HashMap::new(),
        }
    }

    pub fn with_module_path(&self, module_path: String) -> Self {
        Context {
            module_path,
            file_path: self.file_path.clone(),
            file_content: self.file_content.clone(),
            fn_defs: self.fn_defs.clone(),
            mod_defs: self.mod_defs.clone(),
            var_defs: self.var_defs.clone(),
            resolved_type_defs: self.resolved_type_defs.clone(),
        }
    }

    pub fn throw_custom(&self, spanned: &dyn Spanned, message: &str) -> String {
        let filepath = self.file_path.clone();
        let content = self.file_content.clone();
        let source_span = spanned.span();

        ErrorMessage::new(filepath, content, message.to_string(), source_span.from).print();

        panic!()
    }

    pub fn add_fn(&mut self, path: &String, ast_fn: &ast::Fn) -> &mut Self {
        if !self.fn_defs.contains_key(path) {
            self.fn_defs.insert(path.clone(), ast_fn.clone());
            self
        } else {
            self.throw_custom(
                ast_fn,
                &format!("The function {path} has already been defined previously."),
            );
            panic!();
        }
    }

    pub fn add_mod(&mut self, path: &String, ast_mod: &ast::Mod) -> &mut Self {
        if !self.mod_defs.contains_key(path) {
            self.mod_defs.insert(path.clone(), ast_mod.clone());
            self
        } else {
            self.throw_custom(
                ast_mod,
                &format!("The function {path} has already been defined previously."),
            );
            panic!();
        }
    }

    pub fn add_var(&mut self, path: &String, ast_var: &ast::Let) -> &mut Self {
        if !self.var_defs.contains_key(path) {
            self.var_defs.insert(path.clone(), ast_var.clone());
            self
        } else {
            self.throw_custom(
                ast_var,
                &format!("The variable {path} has already been defined previously."),
            );
            panic!();
        }
    }

    pub fn add_resolved_type(
        &mut self,
        path: &String,
        ast_resolved_type: &InferedType,
    ) -> &mut Self {
        self.resolved_type_defs
            .insert(path.clone(), ast_resolved_type.clone());
        self
    }

    pub fn find_method(
        &mut self,
        method_name: &String,
        type_: ast::Ident,
    ) -> Option<(&String, &ast::Fn)> {
        for (path, fn_def) in self.fn_defs.iter() {
            if fn_def.args.len() < 1 {
                continue;
            }

            if &fn_def.id.string != method_name {
                continue;
            }

            if let Type::Ident(v) = &fn_def.args[0].input {
                if v.string == type_.string || LOWERCASED.is_match(&v.string) {
                    return Some((path, fn_def));
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }

        None
    }

    pub fn find_fn(&mut self, path: String) -> Option<(&String, &ast::Fn)> {
        for (path_, fn_def) in self.fn_defs.iter() {
            if path_.starts_with(&path) {
                return Some((path_, fn_def));
            }
        }

        None
    }

    pub fn find_mod(&mut self, path: String) -> Option<(&String, &ast::Mod)> {
        for (path_, mod_def) in self.mod_defs.iter() {
            if path_.starts_with(&path) {
                return Some((path_, mod_def));
            }
        }

        None
    }

    pub fn find_var(&mut self, path: String) -> Option<(&String, &ast::Let)> {
        for (path_, var_def) in self.var_defs.iter() {
            if path_.starts_with(&path) {
                return Some((path_, var_def));
            }
        }

        None
    }
    pub fn merge(&mut self, other: &mut Context) -> &mut Self {
        for (path, fn_def) in &other.fn_defs {
            if !self.fn_defs.contains_key(path) {
                self.fn_defs.insert(path.clone(), fn_def.clone());
            }
        }

        for (path, mod_def) in &other.mod_defs {
            if !self.mod_defs.contains_key(path) {
                self.mod_defs.insert(path.clone(), mod_def.clone());
            }
        }

        self
    }
}
