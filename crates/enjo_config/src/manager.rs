use crate::config::Config;
use home::home_dir;
use std::{env, fs, path::Path};

#[derive(Debug)]
pub enum ManagerLoadError {
    FileNotFound,
    BadStructure,
}

#[derive(Debug)]
pub enum ManagerWriteError {
    WriteFailed,
    FormatFailed,
}

pub struct Manager;
impl Manager {
    pub fn get_config_path() -> String {
        home_dir().unwrap().join(".enjo").join("config.toml").display().to_string()
    }

    pub fn get_config_dir_path() -> String {
        home_dir().unwrap().join(".enjo").display().to_string()
    }

    pub fn get_home_path() -> String {
        home_dir().unwrap().display().to_string()
    }

    pub fn check_exists() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }

    pub fn make_default() -> Config {
        let mut default_config: Config = Config::default();
        default_config.options.path = Self::get_home_path();

        match env::consts::OS.to_string().as_str() {
            "windows" => {
                default_config.programs.editor = String::from("code");
                default_config.programs.shell = String::from("pwsh");
            }
            "linux" => {
                default_config.programs.editor = String::from("nvim");
                default_config.programs.shell = String::from("bash");
            }
            "freebsd" => {
                default_config.programs.editor = String::from("nvim");
                default_config.programs.shell = String::from("bash");
            }
            "netbsd" => {
                default_config.programs.editor = String::from("nvim");
                default_config.programs.shell = String::from("bash");
            }
            "macos" => {
                default_config.programs.editor = String::from("code");
                default_config.programs.shell = String::from("zsh");
            }
            _ => panic!("Unknown platform detected."),
        };

        if let Ok(editor) = env::var("EDITOR") {
            default_config.programs.editor = editor;
        }
        if let Ok(shell) = env::var("SHELL") {
            default_config.programs.shell = shell;
        }

        default_config.options.hide_dots = true;
        default_config
    }

    pub fn load_config() -> Result<Config, ManagerLoadError> {
        match fs::read_to_string(Self::get_config_path()) {
            Ok(content) => match toml::from_str::<Config>(&content) {
                Ok(config) => Ok(config),
                Err(_) => Err(ManagerLoadError::BadStructure),
            },
            Err(_) => Err(ManagerLoadError::FileNotFound),
        }
    }

    pub fn write_config(config: Config) -> Result<(), ManagerWriteError> {
        let dir_path = Self::get_config_dir_path();
        if !Path::new(&dir_path).exists() {
            fs::create_dir(dir_path).unwrap();
        }
        match toml::to_string(&config) {
            Ok(content) => match fs::write(Self::get_config_path(), content) {
                Ok(_) => Ok(()),
                Err(_) => Err(ManagerWriteError::WriteFailed),
            },
            Err(_) => Err(ManagerWriteError::FormatFailed),
        }
    }
}
