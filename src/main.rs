use std::{fs, path::Path, process::exit};

use crate::args::get_args;
use crate::config::Config;
use crate::term::Term;
use platform::Platform;
use utils::Utils;

mod args;
mod config;
mod container;
mod platform;
mod program;
mod term;
mod utils;

#[cfg(test)]
mod tests;

fn main() {
    let args = get_args().get_matches();
    if args.get_flag("version") {
        Utils::display_version();
        exit(0);
    }

    if !Platform::check_exists() {
        let default_config: Config = Config::default();
        Utils::write_config(default_config);

        Term::info("Enjo has generated the default configuration. We recommend you to check it.");
    }

    match args.subcommand() {
        Some(("new", sub)) => {
            let config: Config = Utils::get_config();
            let dir_path: String = config.options.path;

            if !Path::new(&dir_path).exists() {
                Term::fail("A directory with projects does not exist on the file system.");
            }

            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);
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
        Some(("clone", sub)) => {
            let config: Config = Utils::get_config();
            let dir_path: String = config.options.path;

            let repo = sub.get_one::<String>("repo").unwrap().as_str();
            if repo.is_empty() {
                Term::fail("No repository URL provided.");
            }

            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);
            let mut git_args = vec!["clone", repo];
            let branch = sub.get_one::<String>("branch").unwrap();
            let mut name: &str = sub.get_one::<String>("name").unwrap();

            if name.is_empty() {
                name = repo.split(':').collect::<Vec<&str>>()[1];
                name = Path::new(name).file_stem().unwrap().to_str().unwrap();
            }

            if projects.contains(name) {
                Term::fail(format!("Project '{}' already exists.", name).as_str());
            }

            if !branch.is_empty() {
                git_args.push("-b");
                git_args.push(branch);
            }

            git_args.push(name);
            Term::info(format!("Cloning '{}'...", name).as_str());

            Utils::launch_program("git", git_args.iter_mut().map(|i| i.to_string()).collect(), &dir_path);
            Term::done("Done.");
        }
        Some(("open", sub)) => {
            let config: Config = Utils::get_config();
            let dir_path: String = config.options.path;

            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);
            let project_name: &str = sub.get_one::<String>("name").unwrap();

            if project_name.is_empty() {
                Term::fail("Project name is not provided.");
            }

            let is_shell = sub.get_flag("shell");

            let program = match is_shell {
                true => config.programs.shell,
                false => config.programs.editor,
            };

            if program.is_empty() {
                Term::fail("Required program are not specified in configuration file.");
                exit(1)
            }

            if let Some(project) = projects.get(project_name) {
                let project_path = project.get_path_str();
                let path = Path::new(&project_path);
                let proc_args = if is_shell {
                    Vec::new()
                } else {
                    config.options.editor_args.clone()
                };

                let action = if is_shell {
                    "New shell session is starting..."
                } else {
                    "Launching editor..."
                };
                Term::busy(action);

                Utils::launch_program(&program, proc_args, path.to_str().unwrap());

                let end_message = if is_shell {
                    "End of shell session."
                } else {
                    "Editor has been closed."
                };
                Term::info(end_message);

            } else {
                Term::fail("Project not found.");
                exit(1);
            }
        }
        Some(("list", _sub)) => {
            let config: Config = Utils::get_config();
            let dir_path: String = config.options.path;

            if !Path::new(&dir_path).exists() {
                Term::fail("A directory with projects does not exist on the file system.");
            }

            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);
            if projects.is_empty() {
                Term::info("No projects found.");
                exit(0)
            }

            Term::list_title("Your projects:");
            for project in projects.get_vec().iter() {
                if project.name.starts_with('.') && config.options.display_hidden {
                    continue;
                }
                Term::item(project.name.as_str());
            }
        }
        Some(("delete", sub)) => {
            let config: Config = Utils::get_config();
            let dir_path: String = config.options.path;

            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);
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
            Term::info(format!("Removing {}...", name).as_str());
            match fs::remove_dir_all(path.to_str().unwrap()) {
                Ok(_) => Term::done("The project has been deleted."),
                Err(_) => Term::fail("Failed to remove project directory because of the file system error."),
            }
        }
        Some(("config", sub)) => {
            match sub.subcommand() {
                Some(("path", _sub)) => {
                    Term::info(Platform::get_config_path().to_str().unwrap());
                }
                Some(("edit", _sub)) => {
                    let config: Config = Utils::get_config();
                    let editor = config.programs.editor;
                    if editor.is_empty() {
                        Term::fail("Editor program name is not set in the configuration file.")
                    }

                    let path = Platform::get_config_path();
                    let mut editor_args = config.options.editor_args;
                    editor_args.push(path.to_str().unwrap().to_string());
                    Utils::launch_program(editor.as_str(), editor_args, "");
                }
                Some(("reset", _sub)) => {
                    if Term::ask("Do you really want to reset your current configuration?", false) {
                        let new_config: Config = Config::default();
                        Utils::write_config(new_config);
                        Term::done("The configuration has been reset.");
                    } else {
                        Term::info("Aborted.");
                    }
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
