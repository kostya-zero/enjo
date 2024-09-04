use std::collections::HashMap;

pub struct TemplateStorage {
    templates: HashMap<String, Vec<String>>,
}

impl TemplateStorage {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }
}