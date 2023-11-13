use std::{fs, path::Path};

use actions::Actions;
use args::get_args;
use config::Config;
use manager::Manager;
use proc::Proc;
use term::Term;

mod actions;
mod args;
mod config;
mod manager;
mod proc;
mod term;

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
            let config: Config = Actions::get_config().unwrap();
            let mut path: String = config.get_path();

            if path.is_empty() {
                Term::fail("Path option is empty. Please specify it manually.");
            }

            if !Path::new(&path).exists() {
                Term::fail("Directory with projects not found. Check if path set correctly.");
            }

            let name = sub.get_one::<String>("name").unwrap();
            if name.is_empty() {
                Term::fail("New project should have a name.");
            }
            path.push_str(format!("/{}", name).as_str());

            if Path::new(&path).exists() {
                Term::fail("Project with same name already exists.");
            }

            if fs::create_dir(&path).is_err() {
                Term::fail("Failed to create directory for new project.");
            }

            Term::done("Project created.");
        }
        Some(("open", sub)) => {
            let config: Config = Actions::get_config().unwrap();
            let path: String = config.get_path();
            let editor: String = config.get_editor();

            if path.is_empty() {
                Term::fail("Path option is empty. Please specify it manually.");
            }

            if editor.is_empty() {
                Term::fail("Editor option is empty. Please specify it manually.");
            }

            if !Path::new(&path).exists() {
                Term::fail("Directory with projects not found. Check if path set correctly.");
            }

            let project: &str = sub.get_one::<String>("name").unwrap();
            let fullpath = path.clone() + "/" + project;
            if !Path::new(&fullpath).exists() {
                Term::fail("Project not found.");
            }

            let mut proc: Proc = Proc::new(editor.as_str());
            proc.set_args(config.get_editor_args());
            proc.set_cwd(fullpath.as_str());
            proc.run();
        }
        Some(("list", _sub)) => {
            let config: Config = Actions::get_config().unwrap();
            let path: String = config.get_path();

            if !Path::new(&path).exists() {
                Term::fail("Directory with projects not found. Check if path set correctly.");
            }

            let mut projects: Vec<String> = Vec::new();
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if entry.metadata().map(|m| m.is_dir()).unwrap_or(false)
                            && !name.starts_with('.')
                        {
                            projects.push(name.to_owned());
                        }
                    }
                }
            }

            Term::list_title("All projects");
            for i in projects {
                Term::item(i.as_str());
            }
        }
        Some(("delete", sub)) => {
            let config: Config = Actions::get_config().unwrap();
            let path: String = config.get_path();

            if !Path::new(&path).exists() {
                Term::fail("Directory with projects not found. Check if path set correctly.");
            }

            let project: &str = sub.get_one::<String>("name").unwrap();
            let fullpath = path.clone() + "/" + project;
            if !Path::new(&fullpath).exists() {
                Term::fail("Project not found.");
            }

            match fs::remove_dir_all(fullpath) {
                Ok(_) => Term::fail("Failed to remove project directory."),
                Err(_) => Term::done("The project has been deleted."),
            }
        }
        Some(("config", sub)) => {
            match sub.subcommand() {
                Some(("edit", _sub)) => {
                    let config: Config = Actions::get_config().unwrap();
                    let editor: String = config.get_editor();

                    if editor.is_empty() {
                        Term::fail("Editor option is empty. Please specify it manually.");
                    }

                    let mut proc: Proc = Proc::new(editor.as_str());
                    let mut editor_args = config.get_editor_args();
                    let config_path = Manager::get_config_path();
                    editor_args.push(config_path.as_str());
                    proc.set_args(editor_args);
                    proc.run();
                }
                Some(("reset", sub)) => {
                    let yes: bool = sub.get_flag("yes");
                    if !yes {
                        Term::error("You should give your agreement to reset your configuratuion by passing '--yes' argument.");
                        Term::fail("\x1b[4m\x1b[1mYou cant abort this action.\x1b[0m");
                    }

                    let new_config: Config = Manager::make_default();
                    match Manager::write_config(new_config) {
                        Ok(_) => Term::done("Configuration has been set to defaults."),
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
