use std::hash::BuildHasherDefault;
use std::process::Command;
use std::{fs, path::Path, process::exit};

use crate::args::get_args;
use crate::config::Config;
use crate::term::{Dialog, Message};
use library::CloneOptions;
use platform::Platform;
use templates::TemplateStorage;
use utils::Utils;

mod args;
mod config;
mod errors;
mod library;
mod platform;
mod program;
mod templates;
mod term;
mod utils;

#[cfg(test)]
mod tests;

pub fn main() {
    let args = get_args().get_matches();
    if !Platform::check_config_exists() {
        let default_config: Config = Config::default();
        match Config::write(default_config) {
            Ok(_) => Message::info(
                "Enjo has generated the default configuration. Change it according to your needs.",
            ),
            Err(e) => Message::fail(e.to_string().as_str()),
        }
    }

    if !Platform::check_templates_exists() {
        Message::info("Generating template storage...");
        let templates = TemplateStorage::new();
        templates.save().unwrap();
    }

    let config: Config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            Message::fail(e.to_string().as_str());
            exit(1)
        }
    };

    match args.subcommand() {
        Some(("new", sub)) => {
            let dir_path: String = config.options.path;
            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);

            if let Some(name) = sub.get_one::<String>("name") {
                match projects.create(name) {
                    Ok(_) => {},
                    Err(e) => Message::fail(e.to_string().as_str()),
                }

                if let Some(template) = sub.get_one::<String>("template") {
                    if !template.is_empty() {
                        let templates = TemplateStorage::load().unwrap();
                        if let Ok(template) = templates.get(template) {
                            Message::info("Generating project...");
                            let program = match Platform::get_platform() {
                                platform::PlatformName::Windows => "powershell.exe",
                                _ => "sh",
                            };
                            let cwd = Path::new(dir_path.as_str()).join(name);
                            for command in template.iter() {
                                let output = Command::new(program)
                                    .args(["-c", command])
                                    .current_dir(&cwd)
                                    .output();

                                if let Err(e) = output {
                                    Message::fail(&format!("Failed to execute template command {} with error: {}", command, e));
                                }
                            }
                        } else {
                            Message::fail("Template not found.");
                        }
                    }
                }

                Message::done("The project has been created.")
            } else {
                Message::fail("You need to provide a name for your new project.");
            }
        }
        Some(("clone", sub)) => {
            let dir_path = config.options.path.clone();
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

            let projects = Utils::load_projects(&dir_path, config.options.display_hidden);
            match projects.clone(clone_options) {
                Ok(_) => Message::done("The project has been cloned."),
                Err(e) => Message::fail(e.to_string().as_str()),
            }
        }
        Some(("open", sub)) => {
            let dir_path: String = config.options.path;

            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);
            let project_name: &str = sub.get_one::<String>("name").unwrap();

            if project_name.is_empty() {
                Message::fail("Project name is not provided.");
            }

            let is_shell = sub.get_flag("shell");

            let program = match is_shell {
                true => config.shell.program,
                false => config.editor.program,
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
                    config.editor.args.clone()
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
                    config.editor.fork_mode
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
            let dir_path: String = config.options.path;

            if !Path::new(&dir_path).exists() {
                Message::fail("A directory with projects does not exist on the file system.");
            }

            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);
            if projects.is_empty() {
                Message::info("No projects found.");
                exit(0)
            }

            Message::list_title("Your projects:");
            for project in projects.get_vec().iter() {
                let name = project.get_name();
                if name.starts_with('.') && config.options.display_hidden {
                    continue;
                }
                Message::item(name.as_str());
            }
        }
        Some(("rename", sub)) => {
            let dir_path = config.options.path;
            let projects = Utils::load_projects(&dir_path, config.options.display_hidden);

            let name = match sub.get_one::<String>("name") {
                Some(name) if !name.is_empty() => name,
                _ => return Message::fail("You need to provide a name of the project you want to rename."),
            };

            if !projects.contains(name) {
                return Message::fail("Project not found.");
            }

            let new_name = match sub.get_one::<String>("newname") {
                Some(new_name) if !new_name.is_empty() => new_name,
                _ => return Message::fail("You need to provide a new name for the project you want to rename."),
            };

            let system_dirs = [
                ".", "..", "$RECYCLE.BIN", "System Volume Information",
                "msdownld.tmp", ".Trash-1000",
            ];

            if projects.contains(new_name) {
                return Message::fail("A project with the same name has been found.");
            }

            if system_dirs.contains(&new_name.as_str()) {
                return Message::fail("You cannot use the system directory name as the new name.");
            }

            let full_old_path = Path::new(&dir_path).join(name);
            let full_new_path = Path::new(&dir_path).join(new_name);

            match fs::rename(full_old_path, full_new_path) {
                Ok(_) => Message::done(&format!("The project was renamed to '{}'.", new_name)),
                Err(_) => Message::fail("Failed to rename the project."),
            }
        }
        Some(("delete", sub)) => {
            let dir_path: String = config.options.path;

            let projects = Utils::load_projects(dir_path.as_str(), config.options.display_hidden);
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
        Some(("templates", sub)) => {
            match sub.subcommand() {
                Some(("new", _sub)) => {
                    let mut templates = TemplateStorage::load().unwrap();

                    let name = Dialog::ask_string("How do you want to name this template?");
                    if name.is_empty() {
                        Message::fail("Incorrect name for a template.");
                    }

                    let mut commands: Vec<String> = Vec::new();
                    loop {
                        let command = Dialog::ask_string("Enter a command (or just press enter to stop entering commands):");
                        if command.trim().is_empty() {
                            break;
                        } else {
                            commands.push(command.trim().to_string());
                        }
                    }

                    if commands.is_empty() {
                        Message::fail("No commands entered.");
                    }

                    Message::busy("Creating template...");
                    templates.add(&name, commands).unwrap();
                    templates.save().unwrap();
                    Message::done("Template created.");
                }
                Some(("list", _sub)) => {
                    let templates = TemplateStorage::load().unwrap_or_else(|e| {Message::fail(e.to_string().as_str()); exit(1);});
                    if templates.is_empty() {
                        Message::info("No templates found.");
                        exit(0)
                    }

                    Message::list_title("Templates:");
                    for template in templates.get_templates_names().iter() {
                        Message::item(template);
                    }
                }
                Some(("info", sub)) => {
                    let templates = TemplateStorage::load().unwrap();
                    if let Ok(template) = templates.get(sub.get_one::<String>("name").unwrap()) {
                        Message::list_title("Commands of this template:");
                        for command in template.iter() {
                            Message::item(command);
                        }
                    } else {
                        Message::fail("Template not found.");
                    }
                }
                Some(("remove", sub)) => {
                    let mut templates = TemplateStorage::load().unwrap();
                    match templates.remove(sub.get_one::<String>("name").unwrap()) {
                        Ok(_) => {
                            Message::done("Template removed.");
                            templates.save().unwrap();
                        },
                        Err(_) => Message::fail("Template not found."),
                    }
                }
                _ => Message::fail(
                    "Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands.",
                ),
            }
        }
        Some(("config", sub)) => {
            match sub.subcommand() {
                Some(("path", _sub)) => {
                    Message::info(Platform::get_config_path().to_str().unwrap());
                }
                Some(("edit", _sub)) => {
                    let editor = config.editor.program;
                    if editor.is_empty() {
                        Message::fail("Editor program name is not set in the configuration file.")
                    }

                    let path = Platform::get_config_path();
                    let mut editor_args = config.editor.args;
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
        _ => Message::error("This command is not implemented."),
    }
    exit(0);
}
