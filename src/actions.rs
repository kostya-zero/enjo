use std::fs;

use crate::{
    config::Config,
    manager::{Manager, ManagerError},
    term::Term,
};

pub enum UtilsError {
    FetchEntriesError(String),
}

pub struct Utils;
impl Utils {
    pub fn get_config() -> Result<Config, ()> {
        match Manager::load_config() {
            Ok(i) => Ok(i),
            Err(e) => match e {
                ManagerError::FileNotFound => {
                    Term::fail("Cant load configuration file because it's not found.");
                    Err(())
                }
                ManagerError::WriteFailed => {
                    Term::fail("Unknown error.");
                    Err(())
                }
                ManagerError::BadStructure => {
                    Term::fail("Configuration file has a bad structure and cant be serialized.");
                    Err(())
                }
                ManagerError::FormatFailed => {
                    Term::fail("Configuration file has a bad structure and cant be serialized.");
                    Err(())
                }
            },
        }
    }

    pub fn fetch_entries_in_dir(path: &str) -> Result<Vec<String>, UtilsError> {
        let mut new_vec: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if entry.metadata().map(|m| m.is_dir()).unwrap_or(false)
                        && !name.starts_with('.')
                    {
                        new_vec.push(name.to_owned());
                    }
                }
            }
        } else {
            return Err(UtilsError::FetchEntriesError(
                "Cant read directory.".to_string(),
            ));
        }
        Ok(new_vec)
    }
}
