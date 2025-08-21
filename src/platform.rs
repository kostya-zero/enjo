use std::{env, path::PathBuf};

#[derive(PartialEq)]
pub enum PlatformName {
    Windows,
    Linux,
    Mac,
    Unknown,
}

pub fn get_config_path() -> PathBuf {
    get_config_dir_path().join("config.toml")
}

pub fn get_templates_path() -> PathBuf {
    get_config_dir_path().join("templates.json")
}

pub fn get_config_dir_path() -> PathBuf {
    let platform_id = get_platform();
    let user_home = get_user_home();
    match platform_id {
        PlatformName::Windows => user_home.join("AppData").join("Local").join("kanri"),
        _ => user_home.join(".config").join("kanri"),
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
    match get_platform() {
        PlatformName::Windows => {
            PathBuf::from(env::var("USERPROFILE").unwrap_or_else(|_| String::from("C:\\")))
        }
        _ => PathBuf::from(env::var("HOME").unwrap_or_else(|_| String::from("/"))),
    }
}

pub fn get_default_editor() -> String {
    match get_platform() {
        PlatformName::Windows => String::from("code.cmd"),
        PlatformName::Mac => String::from("code"),
        _ => String::from("nvim"),
    }
}

pub fn get_default_shell() -> String {
    match get_platform() {
        PlatformName::Windows => String::from("powershell.exe"),
        PlatformName::Mac => String::from("zsh"),
        _ => String::from("bash"),
    }
}
