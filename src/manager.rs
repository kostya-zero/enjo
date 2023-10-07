use crate::config::Config;
use home::home_dir;
use std::{env, fs, path::Path};

#[derive(Debug)]
pub enum ManagerError {
    FileNotFound,
    WriteFailed,
    BadStructure,
    FormatFailed,
}

pub struct Manager;
impl Manager {
    pub fn get_config_path() -> String {
        home_dir().unwrap().display().to_string() + ".vel.toml"
    }

    pub fn get_home_path() -> String {
        home_dir().unwrap().display().to_string()
    }

    pub fn check_exists() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }

    pub fn make_default() -> Config {
        let mut default_config: Config = Config::default();
        default_config.set_path(Self::get_home_path().as_str());

        match env::consts::OS.to_string().as_str() {
            "windows" => {
                default_config.set_shell("pwsh");
                default_config.set_editor("code");
            }
            "linux" => {
                default_config.set_shell("bash");
                default_config.set_editor("nvim");
            }
            "freebsd" => {
                default_config.set_shell("bash");
                default_config.set_editor("nvim");
            }
            "netbsd" => {
                default_config.set_shell("bash");
                default_config.set_editor("nvim");
            }
            "macos" => {
                default_config.set_shell("zsh");
                default_config.set_editor("code");
            }
            _ => panic!("Unknown platform detected."),
        };
        default_config
    }

    pub fn load_config() -> Result<Config, ManagerError> {
        match fs::read_to_string(Self::get_config_path()) {
            Ok(content) => match toml::from_str::<Config>(&content) {
                Ok(config) => Ok(config),
                Err(_) => Err(ManagerError::BadStructure),
            },
            Err(_) => Err(ManagerError::FileNotFound),
        }
    }

    pub fn write_config(config: Config) -> Result<(), ManagerError> {
        match toml::to_string(&config) {
            Ok(content) => match fs::write(Self::get_config_path(), content) {
                Ok(_) => Ok(()),
                Err(_) => Err(ManagerError::WriteFailed),
            },
            Err(_) => Err(ManagerError::FormatFailed),
        }
    }
}
