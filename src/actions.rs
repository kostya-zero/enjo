use crate::{config::Config, manager::Manager, term::Term};

pub struct Actions;
impl Actions {
    pub fn get_config() -> Result<Config, ()> {
        match Manager::load_config() {
            Ok(i) => Ok(i),
            Err(e) => match e {
                crate::manager::ManagerError::FileNotFound => {
                    Term::fail("Cant load configuration file because it's not found.");
                    Err(())
                }
                crate::manager::ManagerError::WriteFailed => {
                    Term::fail("Unknown error.");
                    Err(())
                },
                crate::manager::ManagerError::BadStructure => {
                    Term::fail("Configuration file has a bad structure and cant be serialized.");
                    Err(())
                },
                crate::manager::ManagerError::FormatFailed => {
                    Term::fail("Configuration file has a bad structure and cant be serialized.");
                    Err(())
                },
            },
        }
    }
}
