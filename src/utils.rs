use std::env;

use crate::config::{Config, ManagerError};
use crate::platform::Platform;
use crate::proc::Proc;
use crate::term::Term;

pub struct Utils;
impl Utils {
    pub fn get_config() -> Option<Config> {
        match Config::load() {
            Ok(i) => Some(i),
            Err(e) => match e {
                ManagerError::FileNotFound => {
                    Term::fail("Cannot load the configuration file because it does not exist on the file system.");
                    None
                }
                ManagerError::BadStructure => {
                    Term::fail("Configuration file has a bad structure and cannot be serialized.");
                    None
                }
                _ => {
                    Term::fail(format!("Unexpected error occured: {:?}", e).as_str());
                    None
                }
            },
        }
    }

    pub fn write_config(config: Config) {
        match Config::write(config) {
            Ok(_) => {}
            Err(e) => match e {
                ManagerError::WriteFailed => Term::fail("Failed to write configuration file."),
                ManagerError::FormatFailed => Term::fail("Failed to format configuration to TOML."),
                _ => {
                    Term::fail(format!("Unexpected error occured: {:?}", e).as_str());
                }
            },
        }
    }

    pub fn make_default() -> Config {
        let mut default_config: Config = Config::default();
        default_config.options.path = Platform::get_user_home().to_str().unwrap().to_string();

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

    pub fn launch_program(program: &str, args: Vec<String>, cwd: &str) {
        let mut proc = Proc::new(program);
        if !cwd.is_empty() {
            proc.set_cwd(cwd);
        }
        proc.set_args(args.iter().map(|i| i.as_str()).collect());
        match proc.run() {
            Ok(_) => {}
            Err(e) => match e {
                crate::proc::ProcError::ExecutableNotFound => {
                    Term::fail("Failed to launch program because executable was not found.")
                }
                crate::proc::ProcError::Interrupted => Term::error("Program was interrupted"),
                crate::proc::ProcError::Other(reason) => {
                    Term::fail(format!("Program failed to launch or failed: {}", reason).as_str())
                }
            },
        }
    }
}
