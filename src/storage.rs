use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::{errors::StorageError, platform::Platform};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Storage {
    templates: HashMap<String, Vec<String>>,
    recent_project: Option<String>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            recent_project: None,
        }
    }

    pub fn load_storage() -> Result<Self, StorageError> {
        if let Ok(content) = fs::read_to_string(Platform::get_templates_path()) {
            let templates: Storage = bincode::deserialize(content.as_bytes()).unwrap();
            Ok(templates)
        } else {
            Err(StorageError::FileSystemError)
        }
    }

    pub fn save_storage(&self) -> Result<(), StorageError> {
        let content = bincode::serialize(&self).unwrap();
        fs::write(Platform::get_templates_path(), content)
            .map_err(|_| StorageError::FileSystemError)
    }

    pub fn add_template(&mut self, name: &str, commands: Vec<String>) -> Result<(), StorageError> {
        if self.templates.contains_key(name) {
            Err(StorageError::AlreadyExists)
        } else {
            self.templates.insert(name.to_string(), commands);
            Ok(())
        }
    }

    pub fn is_templates_empty(&self) -> bool {
        self.templates.is_empty()
    }

    pub fn is_recent_empty(&self) -> bool {
        self.recent_project.is_none()
    }

    pub fn get_template(&self, name: &str) -> Result<&Vec<String>, StorageError> {
        if self.templates.contains_key(name) {
            Ok(self.templates.get(name).unwrap())
        } else {
            Err(StorageError::TemplateNotFound)
        }
    }

    pub fn remove_template(&mut self, name: &str) -> Result<(), StorageError> {
        if self.templates.contains_key(name) {
            self.templates.remove(name);
            Ok(())
        } else {
            Err(StorageError::TemplateNotFound)
        }
    }

    pub fn get_templates_names(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    pub fn set_recent_project(&mut self, name: &str) {
        self.recent_project = Some(name.to_string());
    }

    pub fn get_recent_project(&self) -> Option<String> {
        self.recent_project.clone()
    }
}
