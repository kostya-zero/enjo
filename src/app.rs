use clap::ArgMatches;

use crate::config::Config;

use std::{fs, path::Path, process::exit};

use crate::errors::AppError;
use crate::library::CloneOptions;
use crate::platform::Platform;
use crate::term::{Dialog, Message};
use crate::utils::Utils;
use anyhow::Result;

pub struct App {
    config: Config,
    args: ArgMatches,
}

impl App {
    pub fn new(config: Config, args: ArgMatches) -> Self {
        Self { config, args }
    }

    pub fn run(&self) -> Result<(), AppError> {
        match self.args.subcommand() {
        Some(("new", sub)) => {
            let dir_path: String = self.config.options.path.clone();

            let projects = Utils::load_projects(dir_path.as_str(), self.config.options.display_hidden);
            if let Some(name) = sub.get_one::<String>("name") {
                match projects.create(name) {
                    Ok(_) => Message::done("The project has been created."),
                    Err(e) => return Err(AppError::InternalError(e.into())),
                }
            } else {
                Message::fail("You need to provide a name for your new project.");
            }

        }
        Some(("clone", sub)) => {
            let dir_path = self.config.options.path.clone();
            let mut clone_options = CloneOptions::default();

            if let Some(remote) = sub.get_one::<String>("remote") {
                clone_options.remote = String::from(remote);
            } else {
                Message::fail("You need to provide a remote.");
            }

            if let Some(branch) = sub.get_one::<String>("branch") {
                clone_options.branch = Some(String::from(branch));
            }

            if let Some(name) = sub.get_one::<String>("name") {
                clone_options.name = Some(String::from(name));
            }

            let projects = Utils::load_projects(&dir_path, self.config.options.display_hidden);
            match projects.clone(clone_options) {
                Ok(_) => Message::done("The project has been cloned."),
                Err(e) => Message::fail(e.to_string().as_str()),
            }
        }
        Some(("open", sub)) => {
            let dir_path: String = self.config.options.path.clone();

            let projects = Utils::load_projects(dir_path.as_str(), self.config.options.display_hidden);
            let project_name: &str = sub.get_one::<String>("name").unwrap();

            if project_name.is_empty() {
                Message::fail("Project name is not provided.");
            }

            let is_shell = sub.get_flag("shell");

            let program = match is_shell {
                true => self.config.shell.program.clone(),
                false => self.config.editor.program.clone(),
            };

            if program.is_empty() {
                Message::fail("Required program are not specified in configuration file.");
                exit(1)
            }

            if let Some(project) = projects.get(project_name) {
                let project_path = project.get_path_str();
                let proc_args = if is_shell {
                    Vec::new()
                } else {
                    self.config.editor.args.clone()
                };

                let action = if is_shell {
                    "New shell session is starting..."
                } else {
                    "Launching editor..."
                };
                Message::busy(action);

                let fork_mode = if is_shell {
                    false
                } else {
                    self.config.editor.fork_mode
                };

                Utils::launch_program(&program, proc_args, &project_path, fork_mode);

                if fork_mode {
                    Message::done("Editor launched.");
                    exit(0);
                }

                let end_message = if is_shell {
                    "End of shell session."
                } else {
                    "Editor has been closed."
                };
                Message::info(end_message);

            } else {
                Message::fail("Project not found.");
                exit(1);
            }
        }
        Some(("list", _sub)) => {
            let dir_path: String = self.config.options.path.clone();

            if !Path::new(&dir_path).exists() {
                Message::fail("A directory with projects does not exist on the file system.");
            }

            let projects = Utils::load_projects(dir_path.as_str(), self.config.options.display_hidden);
            if projects.is_empty() {
                Message::info("No projects found.");
                exit(0)
            }

            Message::list_title("Your projects:");
            for project in projects.get_vec().iter() {
                let name = project.get_name();
                if name.starts_with('.') && self.config.options.display_hidden {
                    continue;
                }
                Message::item(name.as_str());
            }
        }
        Some(("rename", sub)) => {
            let dir_path = self.config.options.path.clone();
            let projects = Utils::load_projects(&dir_path, self.config.options.display_hidden);

            let name = match sub.get_one::<String>("name") {
                Some(name) if !name.is_empty() => name,
                _ => return Err(AppError::TextError("Project name is not provided.".to_string())),
            };

            if !projects.contains(name) {
                return Err(AppError::TextError("Project not found.".to_string()));
            }

            let new_name = match sub.get_one::<String>("newname") {
                Some(new_name) if !new_name.is_empty() => new_name,
                _ => return Err(AppError::TextError("New project name is not provided.".to_string())),
            };

            let system_dirs = [
                ".", "..", "$RECYCLE.BIN", "System Volume Information",
                "msdownld.tmp", ".Trash-1000",
            ];

            if projects.contains(new_name) {
                return Err(AppError::TextError("A project with the same name has been found.".to_string()));
            }

            if system_dirs.contains(&new_name.as_str()) {
                return Err(AppError::TextError(
                    "You cannot use the system directory name as the new name.".to_string(),
                ));
            }

            let full_old_path = Path::new(&dir_path).join(name);
            let full_new_path = Path::new(&dir_path).join(new_name);

            match fs::rename(full_old_path, full_new_path) {
                Ok(_) => Message::done(&format!("The project was renamed to '{}'.", new_name)),
                Err(_) => Message::fail("Failed to rename the project."),
            }
        }
        Some(("delete", sub)) => {
            let dir_path: String = self.config.options.path.clone();

            let projects = Utils::load_projects(dir_path.as_str(), self.config.options.display_hidden);
            let name = sub.get_one::<String>("name").unwrap();
            if name.is_empty() {
                Message::fail("You need to provide a name of the project you want to delete.");
            }

            if !projects.contains(name) {
                Message::fail("Project not found.");
            }

            let project = projects.get(name).unwrap();
            if !project.is_empty() && !Dialog::ask("Do you want to delete this project?", false) {
                Message::info("Aborting.");
                exit(0);
            }

            let path = Path::new(&dir_path).join(name);
            Message::info(format!("Removing {}...", name).as_str());
            match fs::remove_dir_all(path.to_str().unwrap()) {
                Ok(_) => Message::done("The project has been deleted."),
                Err(_) => Message::fail("Failed to remove project directory because of the file system error."),
            }
        }
        Some(("config", sub)) => {
            match sub.subcommand() {
                Some(("path", _sub)) => {
                    Message::info(Platform::get_config_path().to_str().unwrap());
                }
                Some(("edit", _sub)) => {
                    let editor = self.config.editor.program.clone();
                    if editor.is_empty() {
                        Message::fail("Editor program name is not set in the configuration file.")
                    }

                    let path = Platform::get_config_path();
                    let mut editor_args = self.config.editor.args.clone();
                    editor_args.push(path.to_str().unwrap().to_string());
                    Utils::launch_program(editor.as_str(), editor_args, "", false);
                }
                Some(("reset", _sub)) => {
                    if Dialog::ask("Do you really want to reset your current configuration?", false) {
                        let new_config: Config = Config::default();
                        Utils::write_config(new_config);
                        Message::done("The configuration has been reset.");
                    } else {
                        Message::info("Aborted.");
                    }
                }
                _ => Message::fail(
                    "Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands.",
                ),
            }
        }
        _ => Message::error("Command not found or it's not implemented yet."),
    }
        Ok(())
    }
}
