use std::path::{Path, PathBuf};

use home::home_dir;

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
        home_dir().unwrap()
    }

    pub fn check_exists() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }
}
