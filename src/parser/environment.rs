use std::collections::HashMap;

use super::ast::Value;

#[derive(Default, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }
    pub fn new_child(&self) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }
    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(v) = self.values.get(name) {
            Some(v)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
    pub fn set(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }
    pub fn contains(&self, name: &str) -> bool {
        if self.values.contains_key(name) {
            true
        } else if let Some(parent) = &self.parent {
            parent.contains(name)
        } else {
            false
        }
    }
}
