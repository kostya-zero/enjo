use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::platform::Platform;

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Config {
    pub options: Options,
    pub programs: Programs,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Options {
    pub path: String,
    pub editor_args: Vec<String>,
    pub hide_dots: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            hide_dots: true,
            path: String::new(),
            editor_args: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Programs {
    pub editor: String,
    pub shell: String,
}

#[derive(Debug)]
pub enum ManagerError {
    WriteFailed,
    FormatFailed,
    FileNotFound,
    BadStructure,
}

impl Config {
    pub fn load() -> Result<Self, ManagerError> {
        match fs::read_to_string(Platform::get_config_path()) {
            Ok(content) => match toml::from_str::<Self>(&content) {
                Ok(config) => Ok(config),
                Err(_) => Err(ManagerError::BadStructure),
            },
            Err(_) => Err(ManagerError::FileNotFound),
        }
    }

    pub fn write(config: Self) -> Result<(), ManagerError> {
        let dir_path = Platform::get_config_dir_path();
        if !Path::new(&dir_path).exists() {
            fs::create_dir(dir_path).unwrap();
        }
        match toml::to_string(&config) {
            Ok(content) => match fs::write(Platform::get_config_path(), content) {
                Ok(_) => Ok(()),
                Err(_) => Err(ManagerError::WriteFailed),
            },
            Err(_) => Err(ManagerError::FormatFailed),
        }
    }
}
