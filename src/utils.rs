use std::process::exit;

use crate::config::{Config, ConfigError};
use crate::container::{Container, ContainerError};
use crate::proc::{Proc, ProcError};
use crate::term::Term;

pub struct Utils;
impl Utils {
    pub fn get_config() -> Config {
        Config::load().unwrap_or_else(|err| {
            match err {
                ConfigError::FileNotFound => {
                    Term::fail("Cannot load the configuration file because it does not exist on the file system.");
                }
                ConfigError::BadStructure => {
                    Term::fail("Configuration file has a bad structure and cannot be serialized.");
                }
                _ => {
                    Term::fail(format!("Unexpected error occured: {:?}", err).as_str());
                }
            };
            exit(1);
        })
    }

    pub fn write_config(config: Config) {
        Config::write(config).unwrap_or_else(|err| match err {
            ConfigError::WriteFailed => Term::fail("Failed to write configuration file."),
            ConfigError::FormatFailed => Term::fail("Failed to format configuration to TOML."),
            _ => {
                Term::fail(format!("Unexpected error occured: {:?}", err).as_str());
            }
        });
    }

    pub fn load_projects(path: &str) -> Container {
        Container::new(path).unwrap_or_else(|err| {
            match err {
                ContainerError::DirectoryNotFound => {
                    Term::fail("A directory with projects does not exist on the file system.");
                }
            };
            exit(1);
        })
    }

    pub fn launch_program(program: &str, args: Vec<String>, cwd: &str) {
        let mut proc = Proc::new(program);
        if !cwd.is_empty() {
            proc.set_cwd(cwd);
        }
        proc.set_args(args.iter().map(|i| i.as_str()).collect());
        if let Err(e) = proc.run() {
            match e {
                ProcError::ExecutableNotFound => {
                    Term::fail(
                        format!(
                            "Failed to launch program because '{}' was not found.",
                            program
                        )
                        .as_str(),
                    );
                }
                ProcError::Interrupted => {
                    Term::error("Program was interrupted");
                }
                ProcError::Other(reason) => {
                    Term::fail(format!("Program failed to launch or failed: {}", reason).as_str());
                }
            }
        }
    }
}
