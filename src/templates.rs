use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::{errors::TemplateStorageError, platform::Platform};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct TemplateStorage {
    templates: HashMap<String, Vec<String>>,
}

impl TemplateStorage {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self, TemplateStorageError> {
        if let Ok(content) = fs::read_to_string(Platform::get_templates_path()) {
            let templates: TemplateStorage = bincode::deserialize(content.as_bytes()).unwrap();
            Ok(templates)
        } else {
            Err(TemplateStorageError::FileSystemError)
        }
    }

    pub fn save(&self) -> Result<(), TemplateStorageError> {
        let content = bincode::serialize(&self).unwrap();
        fs::write(Platform::get_templates_path(), content)
            .map_err(|_| TemplateStorageError::FileSystemError)
    }

    pub fn add(&mut self, name: &str, commands: Vec<String>) -> Result<(), TemplateStorageError> {
        if self.templates.contains_key(name) {
            Err(TemplateStorageError::AlreadyExists)
        } else {
            self.templates.insert(name.to_string(), commands);
            Ok(())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    pub fn get(&self, name: &str) -> Result<&Vec<String>, TemplateStorageError> {
        if self.templates.contains_key(name) {
            Ok(self.templates.get(name).unwrap())
        } else {
            Err(TemplateStorageError::TemplateNotFound)
        }
    }

    pub fn remove(&mut self, name: &str) -> Result<(), TemplateStorageError> {
        if self.templates.contains_key(name) {
            self.templates.remove(name);
            Ok(())
        } else {
            Err(TemplateStorageError::TemplateNotFound)
        }
    }

    pub fn get_templates_names(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }
}
