use crate::configs::config::Config;
use home::home_dir;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum ManagerError {
    WriteFailed,
    FormatFailed,
    FileNotFound,
    BadStructure,
}

pub struct Manager;
impl Manager {
    pub fn get_config_path() -> String {
        let path = Manager::get_user_home();
        path.join(".enjo")
            .join("config.toml")
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_config_dir_path() -> String {
        let path = Manager::get_user_home();
        path.join(".enjo").to_str().unwrap().to_string()
    }

    pub fn get_user_home() -> PathBuf {
        home_dir().unwrap()
    }

    pub fn check_exists() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }

    pub fn make_default() -> Config {
        let mut default_config: Config = Config::default();
        default_config.options.path = Self::get_user_home().to_str().unwrap().to_string();

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

        if env::consts::OS == "windows" {
            if default_config.programs.editor == "code" {
                default_config.programs.editor = "code.cmd".to_owned();
                default_config.options.editor_args = vec![".".to_string()];
            }

            if default_config.programs.editor == "codium" {
                default_config.programs.editor = "codium.cmd".to_owned();
                default_config.options.editor_args = vec![".".to_string()];
            }
        }

        default_config.options.hide_dots = true;
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
        let dir_path = Self::get_config_dir_path();
        if !Path::new(&dir_path).exists() {
            fs::create_dir(dir_path).unwrap();
        }
        match toml::to_string(&config) {
            Ok(content) => match fs::write(Self::get_config_path(), content) {
                Ok(_) => Ok(()),
                Err(_) => Err(ManagerError::WriteFailed),
            },
            Err(_) => Err(ManagerError::FormatFailed),
        }
    }
}
