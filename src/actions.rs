use enjo_configs::config::Config;
use enjo_configs::manager::{Manager, ManagerLoadError, ManagerWriteError};
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

    pub fn resolve_program(program: Option<String>, is_shell: bool) -> Option<String> {
        if let Some(prog) = program {
            if prog.is_empty() {
                if is_shell {
                    Term::fail("Shell is not set in the configuration file.");
                    None
                } else {
                    Term::fail("Editor is not set in the configuration file.");
                    None
                }
            } else {
                Some(prog)
            }
        } else if is_shell {
            Term::fail("Shell is not set in the configuration file.");
            None
        } else {
            Term::fail("Editor is not set in the configuration file.");
            None
        }
    }
}
