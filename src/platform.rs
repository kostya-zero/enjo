use std::{
    env,
    path::{Path, PathBuf},
};

pub struct Platform;
impl Platform {
    pub fn get_config_path() -> String {
        let path = Self::get_user_home();
        path.join(".enjo")
            .join("config.toml")
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_config_dir_path() -> String {
        let path = Self::get_user_home();
        path.join(".enjo").to_str().unwrap().to_string()
    }

    pub fn get_user_home() -> PathBuf {
        match env::consts::OS {
            "windows" => Path::new(&env::var("USERPROFILE").unwrap()).to_path_buf(),
            _ => Path::new(&env::var("HOME").unwrap()).to_path_buf(),
        }
    }

    pub fn check_exists() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }
}
