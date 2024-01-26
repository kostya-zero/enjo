use crate::{
    config::Config,
    manager::{Manager, ManagerError},
    term::Term,
};

pub struct Utils;
impl Utils {
    pub fn get_config() -> Result<Config, ()> {
        match Manager::load_config() {
            Ok(i) => Ok(i),
            Err(e) => match e {
                ManagerError::FileNotFound => {
                    Term::fail("Cant load configuration file because it's not found.");
                    Err(())
                }
                ManagerError::WriteFailed => {
                    Term::fail("Unknown error.");
                    Err(())
                }
                ManagerError::BadStructure => {
                    Term::fail("Configuration file has a bad structure and cant be serialized.");
                    Err(())
                }
                ManagerError::FormatFailed => {
                    Term::fail("Configuration file has a bad structure and cant be serialized.");
                    Err(())
                }
            },
        }
    }

    pub fn resolve_program(program: Option<String>, is_shell: bool) -> String {
        if let Some(prog) = program {
            if prog.is_empty(){
                if is_shell {
                    Term::fail("Shell is not set in configuration file.");
                    String::new()
                } else {
                    Term::fail("Editor is not set in configuration file.");
                    String::new()
                }
            } else {
                prog
            }
        } else if is_shell {
            Term::fail("Shell is not set in configuration file.");
            String::new()
        } else {
            Term::fail("Editor is not set in configuration file.");
            String::new()
        }
    }
}
