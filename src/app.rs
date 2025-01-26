use crate::args::build_cli;
use crate::config::Config;
use crate::library::{CloneOptions, Library};
use crate::platform::{Platform, PlatformName};
use crate::storage::Storage;
use crate::terminal::{Dialog, Message};
use crate::utils::Utils;
use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};
use std::{fs, path::Path, process::exit};

pub fn run() -> Result<()> {
    let args = build_cli().get_matches();

    Utils::check_env()?;

    let config: Config = Config::load()?;
    let mut storage: Storage = Storage::load_storage()?;

    match args.subcommand() {
        Some(("new", sub)) => {
            let projects = Library::new(&config.options.path, config.options.display_hidden)?;

            if let Some(name) = sub.get_one::<String>("name") {
                match projects.create(name) {
                    Ok(_) => {},
                    Err(e) => return Err(anyhow!(e.to_string())),
                }

                if let Some(template) = sub.get_one::<String>("template") {
                    let templates = Storage::load_storage().unwrap();
                    if let Ok(template) = templates.get_template(template) {
                        let count_commands = template.len();
                        let quite = sub.get_flag("quite");
                        Message::info("Generating project from template...");
                        let program = match Platform::get_platform() {
                            PlatformName::Windows => "powershell.exe",
                            _ => "sh",
                        };
                        let cwd = Path::new(&config.options.path).join(name);
                        for command in template.iter() {
                            let mut running_cmd = Command::new(program);
                            running_cmd.args(["-c", command]);
                            if !quite {
                                running_cmd.stdin(Stdio::inherit());
                                running_cmd.stdout(Stdio::inherit());
                                running_cmd.stderr(Stdio::inherit());
                            }
                            running_cmd.current_dir(&cwd);
                            Message::running(format!("Running commands [{}/{}]", template.iter().position(|x| x == command).unwrap() + 1, count_commands).as_str());
                            let output = running_cmd.output();

                            let output_data = match output {
                                Ok(o) => o,
                                Err(e) => return Err(anyhow!(e.to_string())),
                            };

                            if !output_data.status.success() {
                                Message::error(&format!("Template command '{}' failed with exit code: '{}'.", command, output_data.status));
                                match projects.delete(name) {
                                    Ok(_) => {},
                                    Err(_) => return Err(anyhow!("Failed to remove project directory because of the file system error.")),
                                };
                            }
                        }
                    } else {
                        Message::error("Template not found.");
                        match projects.delete(name) {
                            Ok(_) => {},
                            Err(e) => return Err(anyhow!(e.to_string())),
                        };
                        exit(1);
                    }
                }

                Message::done("The project has been created.")
            } else {
                return Err(anyhow!("You need to provide a name for your new project."));
            }
        }
        Some(("clone", sub)) => {
            let mut clone_options = CloneOptions::default();

            if let Some(remote) = sub.get_one::<String>("remote") {
                clone_options.remote = String::from(remote);
            } else {
                return Err(anyhow!("You need to provide a remote."));
            }

            if let Some(branch) = sub.get_one::<String>("branch") {
                clone_options.branch = Some(String::from(branch));
            }

            if let Some(name) = sub.get_one::<String>("name") {
                clone_options.name = Some(String::from(name));
            }

            let projects = Library::new(&config.options.path, config.options.display_hidden)?;
            match projects.clone(clone_options.clone()) {
                Ok(_) => Message::done("The project has been cloned."),
                Err(e) => {
                    return Err(anyhow!(e.to_string()));
                }
            }

            let repo_name = Utils::get_reposiotry_name_from_url(&clone_options.remote);
            if let Some(repo) = repo_name {
                if repo.to_string().starts_with('.') {
                    Message::info("Your project name begins with a dot. It will not be listed unless hidden projects are enabled.");
                }
            }
        }
        Some(("open", sub)) => {
            let projects = Library::new(&config.options.path, config.options.display_hidden)?;
            let project_name: &str = sub.get_one::<String>("name").unwrap();

            let project_final_name =  if project_name == "-" {
                if storage.is_recent_empty() {
                    return Err(anyhow!("You have not opened any project yet."));
                } else {
                    storage.get_recent_project().unwrap()
                }
            } else if config.options.autocomplete {
                Utils::autocomplete(project_name, projects.get_names()).unwrap_or_default()
            } else {
                project_name.to_string()
            };

            if let Ok(project) = projects.get(project_final_name.as_str()) {
                let is_shell = sub.get_flag("shell");
                let program = match is_shell {
                    true => config.shell.program,
                    false => config.editor.program,
                };

                if program.is_empty() {
                    return Err(anyhow!("Required program is not specified in configuration file."));
                }
                storage.set_recent_project(&project_final_name);
                storage.save_storage().unwrap();
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

                match Utils::launch_program(&program, proc_args, project_path, fork_mode) {
                    Ok(_) => {
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
                        exit(0);
                    },
                    Err(e) => return Err(e),
                }
            } else {
                return Err(anyhow!("Project not found."));
            }
        }
        Some(("list", _sub)) => {
            if !Path::new(&config.options.path).exists() {
                return Err(anyhow!("A directory with projects does not exist on the file system."));
            }

            let projects = Library::new(&config.options.path, config.options.display_hidden)?;
            if projects.is_empty() {
                Message::info("No projects found.");
                exit(0)
            }

            Message::list_title("Your projects:");
            for project in projects.get_vec().iter() {
                Message::item(project.get_name());
            }
        }
        Some(("rename", sub)) => {
            let projects = Library::new(&config.options.path, config.options.display_hidden)?;

            let args_name = match sub.get_one::<String>("name") {
                Some(name) if !name.is_empty() => name,
                _ => return Err(anyhow!("You need to provide a name of the project you want to rename.")),
            };

            let name = if config.options.autocomplete {
                &Utils::autocomplete(args_name, projects.get_names()).unwrap_or_default()
            } else {
                args_name
            };

            if !projects.contains(name) {
                return Err(anyhow!("Project not found."));
            }

            let new_name = match sub.get_one::<String>("newname") {
                Some(new_name) if !new_name.is_empty() => new_name,
                _ => return Err(anyhow!("You need to provide a new name for the project you want to rename.")),
            };

            let system_dirs = [
                ".", "..", "$RECYCLE.BIN", "System Volume Information",
                "msdownld.tmp", ".Trash-1000",
            ];

            if projects.contains(new_name) {
                return Err(anyhow!("A project with the same name has been found."));
            }

            if system_dirs.contains(&new_name.as_str()) {
                return Err(anyhow!("You cannot use the system directory name as the new name."));
            }

            let full_old_path = Path::new(&config.options.path).join(name);
            let full_new_path = Path::new(&config.options.path).join(new_name);

            match fs::rename(full_old_path, full_new_path) {
                Ok(_) => Message::done(&format!("The project was renamed to '{}'.", new_name)),
                Err(_) => return Err(anyhow!("Failed to rename the project.")),
            }
        }
        Some(("delete", sub)) => {
            let projects = Library::new(&config.options.path, config.options.display_hidden)?;
            let args_name = sub.get_one::<String>("name").unwrap();
            if args_name.is_empty() {
                return Err(anyhow!("You need to provide a name of the project you want to delete."));
            }

            let name = if config.options.autocomplete {
                &Utils::autocomplete(args_name, projects.get_names()).unwrap_or_default()
            } else {
                args_name
            };

            if !projects.contains(name) {
                return Err(anyhow!("Project not found."));
            }

            let project = projects.get(name).unwrap();
            if !project.is_empty() && !Dialog::ask("Do you want to delete this project?", false) {
                Message::info("Aborting.");
                exit(0);
            }
            Message::busy("Deleting project...");
            match projects.delete(name) {
                Ok(_) => {
                    Message::done("The project has been deleted.");
                },
                Err(_) => {
                    return Err(anyhow!("Failed to remove project directory because of the file system error."));
                },
            }
        }
        Some(("templates", sub)) => {
            match sub.subcommand() {
                Some(("new", _sub)) => {
                    let name = Dialog::ask_string("How do you want to name this template?");
                    if name.is_empty() {
                        return Err(anyhow!("Incorrect name for a template."));
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
                        return Err(anyhow!("No commands entered."));
                    }

                    Message::busy("Creating template...");
                    storage.add_template(&name, commands).unwrap();
                    if storage.save_storage().is_ok() {
                        Message::done("Template created.");
                    } else {
                        return Err(anyhow!("Failed to save templates."));
                    }
                }
                Some(("list", _sub)) => {
                    if storage.is_templates_empty() {
                        Message::info("No templates found.");
                        exit(0)
                    }

                    Message::list_title("Templates:");
                    for template in storage.get_templates_names().iter() {
                        Message::item(template);
                    }
                }
                Some(("info", sub)) => {
                    if let Ok(template) = storage.get_template(sub.get_one::<String>("name").unwrap()) {
                        Message::list_title("Commands of this template:");
                        for command in template.iter() {
                            Message::item(command);
                        }
                    } else {
                        return Err(anyhow!("Template not found."));
                    }
                }
                Some(("remove", sub)) => {
                    match storage.remove_template(sub.get_one::<String>("name").unwrap()) {
                        Ok(_) => {
                            if storage.save_storage().is_ok() {
                                Message::done("Template removed.");
                            } else {
                                return Err(anyhow!("Failed to save templates."));
                            }
                        },
                        Err(_) => return Err(anyhow!("Template not found.")),
                    }
                }
                _ => return Err(anyhow!("Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands.")),
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
                        return Err(anyhow!("Editor program name is not set in the configuration file."));
                    }

                    let path = Platform::get_config_path();
                    let mut editor_args = config.editor.args;
                    editor_args.push(path.to_str().unwrap().to_string());
                    Utils::launch_program(editor.as_str(), editor_args, "", false)?
                }
                Some(("reset", _sub)) => {
                    if Dialog::ask("Do you really want to reset your current configuration?", false) {
                        let new_config: Config = Config::default();
                        Config::write(new_config)?;
                        Message::done("The configuration has been reset.");
                    } else {
                        Message::info("Aborted.");
                    }
                }
                _ => return Err(anyhow!(
                    "Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands.",
                )),
            }
        }
        _ => return Err(anyhow!("This command is not implemented.")),
    }
    Ok(())
}
