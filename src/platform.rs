use std::{
    env,
    path::{Path, PathBuf},
};

pub enum PlatformName {
    Windows,
    Linux,
    Mac,
    Unknown,
}

pub struct Platform;
impl Platform {
    pub fn get_config_path() -> PathBuf {
        Self::get_config_dir_path().join("config.toml")
    }

    pub fn get_config_dir_path() -> PathBuf {
        let platform_id = Self::get_platform();
        let user_home = Self::get_user_home();
        match platform_id {
            PlatformName::Windows => user_home.join("AppData").join("Local").join("enjo"),
            _ => user_home.join(".config").join("enjo"),
        }
    }

    pub fn get_platform() -> PlatformName {
        match env::consts::OS {
            "windows" => PlatformName::Windows,
            "linux" => PlatformName::Linux,
            "macos" => PlatformName::Mac,
            _ => PlatformName::Unknown,
        }
    }

    pub fn get_user_home() -> PathBuf {
        match Self::get_platform() {
            PlatformName::Windows => Path::new(&env::var("USERPROFILE").unwrap()).to_path_buf(),
            _ => Path::new(&env::var("HOME").unwrap()).to_path_buf(),
        }
    }

    pub fn check_exists() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }
}
