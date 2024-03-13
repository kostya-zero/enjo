use std::env;

use crate::configs::config::Config;
use crate::configs::manager::{Manager, ManagerLoadError, ManagerWriteError};
use crate::proc::Proc;
use crate::term::Term;

pub struct Utils;
impl Utils {
    pub fn get_config() -> Option<Config> {
        match Manager::load_config() {
            Ok(i) => Some(i),
            Err(e) => match e {
                ManagerLoadError::FileNotFound => {
                    Term::fail("Cannot load the configuration file because it does not exist on the file system.");
                    None
                }
                ManagerLoadError::BadStructure => {
                    Term::fail("Configuration file has a bad structure and cannot be serialized.");
                    None
                }
            },
        }
    }

    pub fn write_config(config: Config) {
        match Manager::write_config(config) {
            Ok(_) => {}
            Err(e) => match e {
                ManagerWriteError::WriteFailed => Term::fail("Failed to write configuration file."),
                ManagerWriteError::FormatFailed => {
                    Term::fail("Failed to format configuration to TOML.")
                }
            },
        }
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

    pub fn resolve_program(shell: String, editor: String, is_shell: bool) -> String {
        if is_shell {
            if shell.is_empty() {
                Term::fail("Shell parameter in the configuration file is empty.");
                String::new()
            } else {
                shell
            }
        } else if editor.is_empty() {
            Term::fail("Editor parameter in the configuration file is empty.");
            return String::new();
        } else {
            return editor;
        }
    }
}
