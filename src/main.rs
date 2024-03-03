use std::{fs, path::Path, process::exit};

use actions::Actions;
use crate::configs::config::Config;
use crate::configs::manager::Manager;
use crate::args::get_args;
use crate::container::Container;
use crate::term::Term;

mod actions;
mod args;
mod container;
mod proc;
mod term;
mod configs;

fn verify_path(path: String) {
    if path.is_empty() {
        Term::fail("Path is not set in the configuration file.");
        exit(1);
    }

    if !Path::new(&path).exists() {
        Term::fail("Directory with projects not found.");
        exit(1);
    }
}

fn main() {
    if !Manager::check_exists() {
        let default_config: Config = Manager::make_default();
        Actions::write_config(default_config);
    }

    let args = get_args().get_matches();
    match args.subcommand() {
        Some(("new", sub)) => {
            let config: Config = Actions::get_config().unwrap();
            let dir_path: String = config.options.path;
            verify_path(dir_path.clone());

            if !Path::new(&dir_path).exists() {
                Term::fail("A directory with projects does not exist on the file system.");
            }

            let projects = Container::new(&dir_path);
            let name = sub.get_one::<String>("name").unwrap();
            if name.is_empty() {
                Term::fail("You need to provide a name for your new project.");
            }

            if projects.contains(name) {
                Term::fail("Project with this name already exists.");
            }
            
            let new_path = Path::new(&dir_path).join(name);
            match fs::create_dir(new_path) {
                Ok(_) => Term::done("Project created."),
                Err(_) => Term::fail("Failed to create project directory because of file system error."),
            }
            
        }
        Some(("open", sub)) => {
            let config: Config = Actions::get_config().unwrap();
            let dir_path: String = config.options.path;
            verify_path(dir_path.clone());

            let projects = Container::new(&dir_path);
            let project_name = sub.get_one::<String>("name").unwrap();
            let program = Actions::resolve_program(config.programs.shell, config.programs.editor, sub.get_flag("shell"));
            
            if let Some(project) = projects.get(project_name) {
                Term::busy(format!("Launching program ({})...", program).as_str());
                let append: &str = sub.get_one::<String>("append").unwrap();
                let path = Path::new(&project.get_path_str()).join(append);
                let proc_args = if sub.get_flag("shell") {
                    Vec::new()
                } else {
                    config.options.editor_args
                };
                Actions::launch_program(program.as_str(), proc_args.iter().map(|f| f.as_str()).collect(), path.to_str().unwrap());
                Term::done("Program has been closed.");
            } else {
                Term::fail("Project not found.");
                exit(1);
            }
        }
        Some(("list", _sub)) => {
            let config: Config = Actions::get_config().unwrap();
            let dir_path: String = config.options.path;
            verify_path(dir_path.clone());

            if !Path::new(&dir_path).exists() {
                Term::fail("A directory with projects does not exist on the file system.");
            }

            let projects = Container::new(&dir_path);
            if projects.is_empty() {
                Term::info("No projects found.");
                exit(0)
            }

            Term::list_title("Your projects:");
            for project in projects.get_vec().iter() {
                if project.name.starts_with('.') && config.options.hide_dots {
                    continue;
                } 
                Term::item(project.name.as_str());
            }
        }
        Some(("delete", sub)) => {
            let config: Config = Actions::get_config().unwrap();
            let dir_path: String = config.options.path;

            verify_path(dir_path.clone());
            let projects = Container::new(&dir_path);
            let name = sub.get_one::<String>("name").unwrap();
            if name.is_empty() {
                Term::fail("You need to provide a name of the project you want to delete.");
            }

            if name.eq(".") || name.eq("..") {
                Term::fail("Invalid argument value.");
            }

            if !projects.contains(name) {
                Term::fail("Project not found.");
            }

            let path = Path::new(&dir_path).join(name);
            match fs::remove_dir_all(path.to_str().unwrap()) {
                Ok(_) => Term::done("The project has been deleted."),
                Err(_) => Term::fail("Failed to remove project directory because of the file system error."),
            }
        }
        Some(("config", sub)) => {
            match sub.subcommand() {
                Some(("path", _sub)) => {
                    Term::info(Manager::get_config_path().as_str());
                }
                Some(("edit", _sub)) => {
                    let config: Config = Actions::get_config().unwrap();
                    let editor = config.programs.editor;
                    
                    if editor.is_empty() {
                        Term::fail("Editor program name is not set in the configuration file.") 
                    }

                    let path = Manager::get_config_path();
                    let mut editor_args = config.options.editor_args;
                    editor_args.push(path);
                    Actions::launch_program(editor.as_str(), editor_args.iter().map(|f| f.as_str()).collect(), "");
                }
                Some(("reset", sub)) => {
                    let yes: bool = sub.get_flag("yes");
                    if !yes {
                        Term::error("You should give your agreement to reset your configuration by passing '--yes' argument.");
                        exit(1);
                    }

                    let new_config: Config = Manager::make_default();
                    Actions::write_config(new_config);
                }
                _ => Term::fail(
                    "Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands.",
                ),
            }
        }
        _ => Term::error("Command not found or it's not implemented yet."),
    }
    exit(0);
}
