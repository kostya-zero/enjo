use std::{borrow::Cow, env, path::PathBuf};

pub fn config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let base = env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|_| env::var("USERPROFILE").map(|u| PathBuf::from(u).join("AppData\\Local")))
            .unwrap_or_else(|_| PathBuf::from(r"C:\"));
        base.join("kanri")
    }

    #[cfg(target_os = "macos")]
    {
        let home = dirs_next::home_dir().unwrap_or_else(|_| PathBuf::from("/"));
        home.join("Library/Application Support/kanri")
    }

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        if let Some(xdg) = env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join("kanri");
        }
        let home = dirs_next::home_dir().unwrap_or_else(|_| PathBuf::from("/"));
        home.join(".config/kanri")
    }
}

pub fn config_file() -> PathBuf {
    config_dir().join("config.toml")
}
pub fn templates_file() -> PathBuf {
    config_dir().join("templates.json")
}

pub fn default_editor() -> Cow<'static, str> {
    if let Ok(v) = env::var("VISUAL").or_else(|_| env::var("EDITOR")) {
        return Cow::Owned(v);
    }
    #[cfg(target_os = "windows")]
    {
        Cow::Borrowed("code.cmd")
    }
    #[cfg(target_os = "macos")]
    {
        Cow::Borrowed("code")
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        Cow::Borrowed("nvim")
    }
}

pub fn default_shell() -> Cow<'static, str> {
    if let Ok(v) = env::var("SHELL") {
        return Cow::Owned(v);
    }
    #[cfg(target_os = "windows")]
    {
        Cow::Borrowed("powershell.exe")
    }
    #[cfg(target_os = "macos")]
    {
        Cow::Borrowed("zsh")
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        Cow::Borrowed("bash")
    }
}

pub fn default_projects_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(up) = env::var_os("USERPROFILE") {
            return PathBuf::from(up).join("Projects");
        }
        PathBuf::from(r"C:\Projects")
    }

    #[cfg(target_os = "macos")]
    {
        dirs_next::home_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .join("Projects")
    }

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        dirs_next::home_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .join("Projects")
    }
}
