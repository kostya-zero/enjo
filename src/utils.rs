use crate::config::{Config, ConfigError};
use crate::container::{Container, ContainerError};
use crate::proc::Proc;
use crate::term::Term;

pub struct Utils;
impl Utils {
    pub fn get_config() -> Option<Config> {
        match Config::load() {
            Ok(i) => Some(i),
            Err(e) => match e {
                ConfigError::FileNotFound => {
                    Term::fail("Cannot load the configuration file because it does not exist on the file system.");
                    None
                }
                ConfigError::BadStructure => {
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
                ConfigError::WriteFailed => Term::fail("Failed to write configuration file."),
                ConfigError::FormatFailed => Term::fail("Failed to format configuration to TOML."),
                _ => {
                    Term::fail(format!("Unexpected error occured: {:?}", e).as_str());
                }
            },
        }
    }

    pub fn load_projects(path: &str) -> Option<Container> {
        match Container::new(path) {
            Ok(i) => Some(i),
            Err(e) => match e {
                ContainerError::DirectoryNotFound => {
                    Term::fail("A directory with projects does not exist on the file system.");
                    None
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
                crate::proc::ProcError::ExecutableNotFound => Term::fail(
                    format!(
                        "Failed to launch program because '{}' was not found.",
                        program
                    )
                    .as_str(),
                ),
                crate::proc::ProcError::Interrupted => Term::error("Program was interrupted"),
                crate::proc::ProcError::Other(reason) => {
                    Term::fail(format!("Program failed to launch or failed: {}", reason).as_str())
                }
            },
        }
    }
}
