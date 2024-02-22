use enjo_config::config::Config;
use enjo_config::manager::{Manager, ManagerLoadError, ManagerWriteError};
use enjo_tools::proc::Proc;
use enjo_tools::term::Term;

pub struct Actions;
impl Actions {
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

    pub fn launch_program(program: &str, args: Vec<&str>, cwd: &str) {
        let mut proc = Proc::new(program);
        if !cwd.is_empty() {
            proc.set_cwd(cwd);
        }
        proc.set_args(args);
        match proc.run() {
            Ok(_) => {}
            Err(e) => match e {
                enjo_tools::proc::ProcError::ExecutableNotFound => {
                    Term::fail("Failed to launch program because executable was not found.")
                }
                enjo_tools::proc::ProcError::Interrupted => Term::error("Program was interrupted"),
                enjo_tools::proc::ProcError::Other(reason) => {
                    Term::fail(format!("Program failed to launch or failed: {}", reason).as_str())
                }
            },
        }
    }

    pub fn resolve_program(
        shell: String,
        editor: String,
        is_shell: bool,
    ) -> String {
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
