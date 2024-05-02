use crate::platform::{Platform, PlatformName};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub options: Options,
    pub programs: Programs,
}

impl Default for Config {
    fn default() -> Self {
        let mut default_options = Options::default();
        let mut default_programs = Programs::default();
        if Platform::get_platform() == PlatformName::Windows
            && (default_programs.editor == "code" || default_programs.editor == "codium")
        {
            default_programs.editor.push_str(".cmd");
            default_options.editor_args.push(".".to_string());
        }

        Self {
            options: default_options,
            programs: default_programs,
        }
    }
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

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Programs {
    pub editor: String,
    pub shell: String,
}

impl Default for Programs {
    #[allow(unused_assignments)]
    fn default() -> Self {
        let mut new_editor: String = String::new();
        let mut new_shell: String = String::new();
        let current_platform = Platform::get_platform();
        match current_platform {
            PlatformName::Windows => {
                new_editor = String::from("code.cmd");
                new_shell = String::from("pwsh");
            }
            PlatformName::Mac => {
                new_editor = String::from("code");
                new_shell = String::from("zsh");
            }
            _ => {
                new_editor = String::from("nvim");
                new_shell = String::from("bash");
            }
        }
        if let Ok(env_editor) = env::var("EDITOR") {
            new_editor = env_editor;
        }
        if let Ok(env_shell) = env::var("SHELL") {
            new_shell = env_shell;
        }

        if PlatformName::Windows == current_platform
            && (new_editor.contains("code") || new_editor.contains("codium"))
        {
            new_editor.push_str(".cmd");
        }

        Self {
            editor: new_editor,
            shell: new_shell,
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    WriteFailed,
    FormatFailed,
    FileNotFound,
    BadStructure,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        match fs::read_to_string(Platform::get_config_path()) {
            Ok(content) => match toml::from_str::<Self>(&content) {
                Ok(config) => Ok(config),
                Err(_) => Err(ConfigError::BadStructure),
            },
            Err(_) => Err(ConfigError::FileNotFound),
        }
    }

    pub fn write(config: Self) -> Result<(), ConfigError> {
        let dir_path = Platform::get_config_dir_path();
        if !Path::new(&dir_path).exists() {
            fs::create_dir(dir_path).unwrap();
        }
        match toml::to_string(&config) {
            Ok(content) => match fs::write(Platform::get_config_path(), content) {
                Ok(_) => Ok(()),
                Err(_) => Err(ConfigError::WriteFailed),
            },
            Err(_) => Err(ConfigError::FormatFailed),
        }
    }
}
