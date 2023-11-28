use crate::{
    config::Config,
    manager::{Manager, ManagerError},
    projects::{ProjectsContainer, ProjectsError},
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

    pub fn fetch_directory(path: &str) -> Option<ProjectsContainer> {
        match ProjectsContainer::new(path) {
            Ok(container) => Some(container),
            Err(e) => match e {
                ProjectsError::RootNotFound => {
                    Term::fail("Cannot find directory by given path.");
                    None
                }
                ProjectsError::DirReadFailed => {
                    Term::fail("Cannot read directory by given path.");
                    None
                }
            },
        }
    }
}
