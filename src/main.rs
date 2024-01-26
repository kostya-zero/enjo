use std::{fs, path::Path, process::exit};

use args::get_args;
use config::Config;
use container::Container;
use manager::Manager;
use proc::Proc;
use term::Term;
use utils::Utils;

mod args;
mod config;
mod manager;
mod proc;
mod container;
mod term;

mod utils;

fn main() {
    if !Manager::check_exists() {
        let default_config: Config = Manager::make_default();
        match Manager::write_config(default_config) {
            Ok(_) => Term::done("Default configuration generated."),
            Err(e) => match e {
                manager::ManagerError::WriteFailed => {
                    Term::fail("Failed to write default configuration to file.")
                }
                manager::ManagerError::FormatFailed => {
                    Term::fail("Failed to format configuration to TOML.")
                }
                _ => Term::fail("Unknown error occured."),
            },
        }
    }

    let args = get_args().get_matches();
    match args.subcommand() {
        Some(("new", sub)) => {
            let config: Config = Utils::get_config().unwrap();
            if let Some(dir_path) = config.get_path() {
                if dir_path.is_empty() {
                    Term::fail("Path option in configuration is empty.")
                }

                let projects = Container::new(&dir_path);
                if let Some(name) = sub.get_one::<String>("name") {
                    if name.is_empty() {
                        Term::fail("Specify a name for your new project");
                    }

                    if projects.contains(name) {
                        Term::fail("Project with same name already exists.");
                    }

                    let new_path = Path::new(&dir_path).join(name);
                    match fs::create_dir(new_path) {
                        Ok(_) => Term::done("Project created."),
                        Err(_) => Term::fail("Failed to make project directory."),
                    }
                }
            } else {
                Term::fail("Path to projects not set in configuration.");
            }
        }
        Some(("open", sub)) => {
            let config: Config = Utils::get_config().unwrap();
            if let Some(dir_path) = config.get_path() {
                if dir_path.is_empty() {
                    Term::fail("Path option in configuration is empty.")
                }

                let program = if sub.get_flag("shell") {
                    Utils::resolve_program(config.get_shell(), true)
                } else {
                    Utils::resolve_program(config.get_editor(), true)
                };

                let projects = Container::new(&dir_path);
                let project_name = sub.get_one::<String>("name").unwrap();
                if !projects.contains(project_name) {
                    Term::fail("Project not found.");
                }
                if let Some(project) = projects.get(project_name) {
                    let append: &str = sub.get_one::<String>("append").unwrap();
                    let path = Path::new(&project.path).join(append);
                    let mut proc: Proc = Proc::new(program.as_str());
                    proc.set_cwd(path.to_str().unwrap());
                    Term::busy(format!("Launching program ({})...", program).as_str());
                    proc.run();
                    Term::done("Program closed.");
                    exit(0);
                }
            }
        }
        Some(("list", _sub)) => {
            let config: Config = Utils::get_config().unwrap();
            if let Some(dir_path) = config.get_path() {
                if dir_path.is_empty() {
                    Term::fail("Path option in configuration is empty.")
                }
                let projects = Container::new(&dir_path);
                Term::list_title("All projects");
                for project in projects.get_vec().iter() {
                    Term::item(project.name.as_str());
                }
            }
        }
        Some(("delete", sub)) => {
            let config: Config = Utils::get_config().unwrap();
            if let Some(dir_path) = config.get_path() {
                println!("{:?}", dir_path);

                if dir_path.is_empty() {
                    Term::fail("Path option in configuration is empty.")
                }
                let projects = Container::new(&dir_path);
                if let Some(name) = sub.get_one::<String>("name") {
                    if name.eq(".") || name.eq("..") {
                        Term::fail("You cant remove parent or directory with projects.");
                    }

                    if !projects.contains(name) {
                        Term::fail("Project not found.");
                    }

                    let path = Path::new(&dir_path).join(name);
                    match fs::remove_dir_all(path.to_str().unwrap()) {
                        Ok(_) => Term::done("The project has been deleted."),
                        Err(_) => Term::fail("Failed to remove project directory"),
                    }
                }
            }
        }
        Some(("config", sub)) => {
            match sub.subcommand() {
                Some(("path", _sub)) => {
                    Term::info(Manager::get_config_path().as_str());
                }
                Some(("edit", _sub)) => {
                    let config: Config = Utils::get_config().unwrap();

                    if let Some(editor) = config.get_editor() {
                        if editor.is_empty() {
                           Term::fail("Editor option is empty.") 
                        }

                        let path = Manager::get_config_path();
                        if let Some(mut editor_args) = config.get_editor_args() {
                            let mut proc: Proc = Proc::new(editor.as_str());
                            editor_args.push(&path);
                            proc.set_args(editor_args);
                            proc.run();
                        }
                    }
                }
                Some(("reset", sub)) => {
                    let yes: bool = sub.get_flag("yes");
                    if !yes {
                        Term::error("You should give your agreement to reset your configuratuion by passing '--yes' argument.");
                        Term::fail("\x1b[4m\x1b[1mYou cant abort this action.\x1b[0m");
                    }

                    let new_config: Config = Manager::make_default();
                    match Manager::write_config(new_config) {
                        Ok(_) => Term::done("Configuration values have been set to default."),
                        Err(e) => match e {
                            manager::ManagerError::WriteFailed => {
                                Term::fail("Failed to write default configuration to file.")
                            }
                            manager::ManagerError::FormatFailed => {
                                Term::fail("Failed to format configuration to TOML.")
                            }
                            _ => Term::fail("Unknown error occured."),
                        },
                    }
                }
                _ => Term::fail(
                    "Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands.",
                ),
            }
        }
        _ => Term::error("Command not found or it's not implemented yet."),
    }
}
