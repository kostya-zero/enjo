use crate::{
    errors::ConfigError,
    platform::{Platform, PlatformName},
};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

#[derive(Deserialize, Serialize, Clone)]
pub struct Options {
    pub projects_directory: String,
    pub display_hidden: bool,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Config {
    pub options: Options,
    pub editor: EditorOptions,
    pub shell: ShellOptions,
    pub recent: Recent,
    pub autocomplete: Autocomplete,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Autocomplete {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Recent {
    pub enabled: bool,
    pub recent_project: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            projects_directory: Platform::get_user_home().to_str().unwrap().to_string(),
            display_hidden: false,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EditorOptions {
    pub program: String,
    pub fork_mode: bool,
    pub args: Vec<String>,
}

impl Default for EditorOptions {
    fn default() -> Self {
        let mut new_editor: String = Platform::get_default_editor();
        let mut new_args: Vec<String> = Vec::new();
        let mut fork_mode = false;

        if let Ok(env_editor) = env::var("EDITOR") {
            new_editor = env_editor;
        }

        match new_editor.as_str() {
            "code" | "codium" | "windsurf" => {
                new_args.push(".".to_string());
                fork_mode = true;
                if PlatformName::Windows == Platform::get_platform() {
                    new_editor.push_str(".cmd");
                }
            }
            "zed" => {
                fork_mode = true;
                new_args.push(".".to_string());
            }
            _ => {}
        }

        Self {
            program: new_editor,
            fork_mode,
            args: new_args,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ShellOptions {
    pub program: String,
}

impl Default for ShellOptions {
    fn default() -> Self {
        let shell_program = env::var("SHELL").unwrap_or_else(|_| Platform::get_default_shell());
        Self {
            program: shell_program,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Platform::get_config_path();
        let content = fs::read_to_string(config_path).map_err(|_| ConfigError::FileNotFound)?;
        toml::from_str(&content).map_err(|_| ConfigError::BadStructure)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let dir_path = Platform::get_config_dir_path();
        if !Path::new(&dir_path).exists() {
            fs::create_dir(&dir_path).map_err(|_| ConfigError::WriteFailed)?;
        }
        let content = toml::to_string(self).map_err(|_| ConfigError::FormatFailed)?;
        fs::write(Platform::get_config_path(), content).map_err(|_| ConfigError::WriteFailed)
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
