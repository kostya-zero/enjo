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
                    Term::fail("Configuration file has a bad structure and cannot be deserialized.");
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

    pub fn load_projects(path: &str, display_hidden: bool) -> Container {
        Container::new(path, display_hidden).unwrap_or_else(|err| {
            match err {
                ContainerError::DirectoryNotFound => {
                    Term::fail("A directory with projects does not exist on the file system.");
                }
            };
            exit(1);
        })
    }

    pub fn display_version() {
        let build_type = if cfg!(debug_assertions) {
            "dev"
        } else {
            "release"
        };

        let version = env!("CARGO_PKG_VERSION");
        Term::info(format!("v{} ({})", version, build_type).as_str()    );

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
