use std::{fs, path::Path};

use actions::Utils;
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
            let config: Config = Utils::get_config().unwrap();
            if let Some(dir_path) = config.get_path() {
                let path = Path::new(&dir_path);
                if !path.exists() {
                    Term::fail("Directory with projects not exists.");
                }

                if let Some(name) = sub.get_one::<String>("name") {
                    if name.is_empty() {
                        Term::fail("Specify a name for your new project.");
                    }

                    let new_path = path.join(name);

                    if new_path.exists() {
                        Term::fail("Project with the same name already exists.");
                    }

                    match fs::create_dir(&new_path) {
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
                if let Some(editor) = config.get_editor() {
                    if dir_path.is_empty() {
                        Term::fail("Path option is empty.");
                    }

                    if editor.is_empty() {
                        Term::fail("Editor option is empty.");
                    }

                    let path: &Path = Path::new(&dir_path);
                    if !path.exists() {
                        Term::fail("Directory with projects not exists.");
                    }

                    if let Some(project) = sub.get_one::<String>("name") {
                        let new_path = path.join(project);
                        if !new_path.exists() {
                            Term::fail("Project not found.");
                        }
                        let editor_args = if let Some(opt_args) = config.get_editor_args() {
                            opt_args
                        } else {
                            Vec::new()
                        };
                        let mut proc: Proc = Proc::new(editor.as_str());
                        proc.set_args(editor_args);
                        proc.set_cwd(new_path.to_str().unwrap());
                        Term::busy(format!("Launching editor ({})...", editor).as_str());
                        proc.run();
                        Term::done("Editor closed.");
                    }
                }
            }
        }
        Some(("list", _sub)) => {
            let config: Config = Utils::get_config().unwrap();
            if let Some(dir_path) = config.get_path() {
                let path = Path::new(&dir_path);
                if !path.exists() {
                    Term::fail("Directory with projects not exists.");
                }

                match Utils::fetch_entries_in_dir(path.to_str().unwrap()) {
                    Ok(projects) => {
                        Term::list_title("All projects");
                        for project in projects {
                            Term::item(&project);
                        }
                    },
                    Err(e) => match e {
                        actions::UtilsError::FetchEntriesError(info) => Term::fail(format!("Failed to fetch projects. {info}").as_str()),
                    },
                }
            }
        }
        Some(("delete", sub)) => {
            let config: Config = Utils::get_config().unwrap();
            if let Some(dir_path) = config.get_path() {
                let path = Path::new(&dir_path);

                if !path.exists() {
                    Term::fail("Directory with projects not exists.");
                }

                if let Some(name) = sub.get_one::<String>("name") {
                    let new_path = path.join(name);

                    if !new_path.exists() {
                        Term::fail("Project not found.");
                    }

                    match fs::remove_dir_all(new_path.to_str().unwrap()) {
                        Ok(_) => Term::done("The project has been deleted."),
                        Err(_) => Term::fail("Failed to remove project directory"),
                    }
                }
            }
        }
        Some(("config", sub)) => {
            match sub.subcommand() {
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
