use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, fs, path::PathBuf};

use crate::{errors::StorageError, platform::Platform};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Storage {
    templates: HashMap<String, Vec<Cow<'static, str>>>,
    recent_project: Option<Cow<'static, str>>,
    #[serde(skip)]
    storage_path: Option<PathBuf>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            templates: HashMap::with_capacity(10),
            recent_project: None,
            storage_path: None,
        }
    }

    pub fn load_storage() -> Result<Self, StorageError> {
        let path = Platform::get_storage_path();
        match fs::read_to_string(&path) {
            Ok(content) => {
                let mut storage: Storage = bincode::deserialize(content.as_bytes())
                    .map_err(|_| StorageError::DeserializationError)?;
                storage.storage_path = Some(path);
                Ok(storage)
            }
            Err(_) => Ok(Self::new()), // Return empty storage if file doesn't exist
        }
    }

    pub fn save_storage(&self) -> Result<(), StorageError> {
        let content = bincode::serialize(&self).map_err(|_| StorageError::SerializationError)?;

        if let Some(path) = &self.storage_path {
            // Ensure parent directory exists
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|_| StorageError::FileSystemError)?;
            }

            fs::write(path, content).map_err(|_| StorageError::FileSystemError)
        } else {
            let path = Platform::get_storage_path();
            fs::write(path, content).map_err(|_| StorageError::FileSystemError)
        }
    }

    pub fn add_template(&mut self, name: &str, commands: Vec<String>) -> Result<(), StorageError> {
        if self.templates.contains_key(name) {
            return Err(StorageError::AlreadyExists);
        }

        if commands.iter().any(|cmd| cmd.trim().is_empty()) {
            return Err(StorageError::InvalidCommand);
        }

        self.templates
            .insert(name.to_string(), commands.into_iter().map(Cow::from).collect());
        Ok(())
    }

    pub fn is_templates_empty(&self) -> bool {
        self.templates.is_empty()
    }

    pub fn is_recent_empty(&self) -> bool {
        self.recent_project.is_none()
    }

    pub fn remove_template(&mut self, name: &str) -> Result<(), StorageError> {
        self.templates
            .remove(name)
            .map(|_| ())
            .ok_or(StorageError::TemplateNotFound)
    }

    pub fn get_templates_names(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    pub fn get_template(&self, name: &str) -> Result<&Vec<Cow<'static, str>>, StorageError> {
        self.templates
            .get(name)
            .ok_or(StorageError::TemplateNotFound)
    }

    pub fn set_recent_project(&mut self, name: &str) {
        self.recent_project = Some(name.to_string().into());
    }

    pub fn get_recent_project(&self) -> Option<Cow<'static, str>> {
        self.recent_project.clone()
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        // Attempt to save any pending changes when the storage is dropped
        let _ = self.save_storage();
    }
}
