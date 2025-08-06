use crate::platform::{Platform, PlatformName};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to write configuration file.")]
    WriteFailed,

    #[error("Failed to format configuration to TOML.")]
    FormatFailed,

    #[error("Cannot find configuration file.")]
    FileNotFound,

    #[error("Configuration file has a bad structure and cannot be deserialized.")]
    BadStructure,
}

#[derive(Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct Config {
    pub options: Options,
    pub editor: EditorOptions,
    pub shell: ShellOptions,
    pub recent: Recent,
    pub autocomplete: Autocomplete,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Options {
    pub projects_directory: String,
    pub display_hidden: bool,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Autocomplete {
    pub enabled: bool,
    pub always_accept: bool,
}

impl Default for Autocomplete {
    fn default() -> Self {
        Self {
            enabled: true,
            always_accept: true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Recent {
    pub enabled: bool,
    pub recent_project: String,
}

impl Default for Recent {
    fn default() -> Self {
        Self {
            enabled: true,
            recent_project: String::new(),
        }
    }
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
#[serde(default)]
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
            "code" | "code-insiders" | "codium" | "code-oss" | "windsurf" => {
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
#[serde(default)]
pub struct ShellOptions {
    pub program: String,
    pub args: Vec<String>,
}

impl Default for ShellOptions {
    fn default() -> Self {
        let program = env::var("SHELL").unwrap_or_else(|_| Platform::get_default_shell());
        let args = match program.as_str() {
            "powershell.exe" | "powershell" | "pwsh.exe" | "pwsh" => {
                vec!["-NoLogo".to_string(), "-Command".to_string()]
            }
            "cmd" | "cmd.exe" => vec!["/C".to_string()],
            "zsh" | "bash" | "fish" | "sh" => vec!["-c".to_string()],
            _ => vec!["-c".to_string()],
        };
        Self { program, args }
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
