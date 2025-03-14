use crate::args::build_cli;
use crate::program::Program;
use crate::config::Config;
use crate::library::{CloneOptions, Library};
use crate::platform::Platform;
use crate::storage::Storage;
use crate::terminal::{Dialog, Message, create_spinner};
use crate::utils::Utils;
use anyhow::{Result, anyhow, bail};
use std::{borrow::Cow, path::Path};

pub fn run() -> Result<()> {
    let args = build_cli().get_matches();

    Utils::check_env()?;

    match args.subcommand() {
        Some(("new", sub)) => {
            let config: Config = Config::load()?;
            let projects = Library::new(&config.options.path, config.options.display_hidden)?;

            let name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("You need to provide a name for your new project."))?;

            projects.create(name)?;

            if let Some(template_name) = sub.get_one::<String>("template") {
                if let Err(e) = Utils::apply_template(
                    template_name,
                    name,
                    &config.options.path,
                    sub.get_flag("quite"),
                ) {
                    Message::error("Failed to apply template. Cleaning up...");
                    if let Err(cleanup_err) = projects.delete(name) {
                        bail!(
                            "Template application failed: {}. Additionally, cleanup failed: {}",
                            e,
                            cleanup_err
                        )
                    }
                    return Err(e);
                }
            }

            Message::done("The project has been created.")
        }
        Some(("clone", sub)) => {
            let config: Config = Config::load()?;
            let remote = sub
                .get_one::<String>("remote")
                .filter(|s| !s.trim().is_empty())
                .map(String::from)
                .ok_or_else(|| anyhow!("You need to provide a remote URL."))?;

            let branch = sub
                .get_one::<String>("branch")
                .filter(|s| !s.trim().is_empty())
                .map(String::from);

            let name = sub
                .get_one::<String>("name")
                .filter(|s| !s.trim().is_empty())
                .map(String::from);

            let clone_options = CloneOptions {
                remote,
                branch,
                name,
            };

            let projects = Library::new(&config.options.path, config.options.display_hidden)?;
            match projects.clone(&clone_options) {
                Ok(_) => Message::done("The project has been cloned."),
                Err(e) => bail!(e.to_string()),
            }
        }
        Some(("open", sub)) => {
            let config: Config = Config::load()?;
            let mut storage: Storage = Storage::load_storage()?;
            let projects = Library::new(&config.options.path, config.options.display_hidden)?;
            let project_name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("The project name is not provided."))?;

            let name = if project_name == "-" {
                if storage.is_recent_empty() {
                    bail!("No project was opened recently.")
                }
                storage.get_recent_project().unwrap()
            } else if config.options.autocomplete {
                Cow::from(
                    Utils::autocomplete(project_name, projects.get_names()).unwrap_or_default(),
                )
            } else {
                Cow::from(project_name)
            };

            if let Ok(project) = projects.get(name.as_ref()) {
                let is_shell = sub.get_flag("shell");
                let program = match is_shell {
                    true => config.shell.program,
                    false => config.editor.program,
                };

                if program.is_empty() {
                    bail!("Required program is not specified in configuration file.");
                }

                if name != storage.get_recent_project().unwrap_or(Cow::from("")) {
                    storage.set_recent_project(&name);
                    storage.save_storage().unwrap();
                }

                let project_path = project.get_path_str();
                let proc_args = if is_shell {
                    Vec::new()
                } else {
                    config.editor.args.clone()
                };

                if is_shell {
                    Message::busy("New shell session is starting...");
                } else {
                    Message::busy("Launching editor...");
                };

                let fork_mode = if is_shell {
                    false
                } else {
                    config.editor.fork_mode
                };

                match Program::launch_program(&program, proc_args, project_path, fork_mode) {
                    Ok(_) => {
                        if fork_mode {
                            Message::done("Editor launched.");
                            return Ok(());
                        }
                        let end_message = if is_shell {
                            "End of shell session."
                        } else {
                            "Editor has been closed."
                        };
                        Message::info(end_message);
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            } else {
                bail!("Project not found.");
            }
        }
        Some(("list", _sub)) => {
            let config: Config = Config::load()?;
            if !Path::new(&config.options.path).exists() {
                bail!("Directory with projects does not exist on the file system.");
            }

            let projects = Library::new(&config.options.path, config.options.display_hidden)?;
            if projects.is_empty() {
                Message::info("No projects found.");
                return Ok(());
            }

            Message::title("Your projects:");
            for project in projects.get_vec().iter() {
                Message::item(project.get_name());
            }
        }
        Some(("rename", sub)) => {
            let config: Config = Config::load()?;
            let projects = Library::new(&config.options.path, config.options.display_hidden)?;

            let args_name = match sub.get_one::<String>("name") {
                Some(name) if !name.is_empty() => name,
                _ => {
                    bail!("You need to provide a name of the project you want to rename.");
                }
            };

            let name = if config.options.autocomplete {
                &Utils::autocomplete(args_name, projects.get_names()).unwrap_or_default()
            } else {
                args_name
            };

            let new_name = match sub.get_one::<String>("newname") {
                Some(new_name) if !new_name.is_empty() => new_name,
                _ => {
                    bail!("New name is not provided.");
                }
            };

            match projects.rename(name, new_name) {
                Ok(_) => Message::done(&format!("The project was renamed to '{}'.", new_name)),
                Err(e) => bail!(e.to_string()),
            }
        }
        Some(("delete", sub)) => {
            let config: Config = Config::load()?;
            let projects = Library::new(&config.options.path, config.options.display_hidden)?;

            let args_name = match sub.get_one::<String>("name") {
                Some(name) if !name.is_empty() => name,
                _ => {
                    bail!("You need to provide a name of the project you want to delete.");
                }
            };

            let project_name = if config.options.autocomplete {
                Utils::autocomplete(args_name, projects.get_names()).unwrap_or_default()
            } else {
                args_name.to_string()
            };

            let project = projects
                .get(&project_name)
                .map_err(|_| anyhow!("Project not found."))?;

            let force_delete = sub.get_flag("force");
            let is_empty = project.is_empty()?;

            if !is_empty
                && !force_delete
                && !Dialog::ask("Are you sure you want to delete this project?", false)
            {
                Message::info("Aborting.");
                return Ok(());
            }

            let spinner = create_spinner();
            spinner.set_message("Deleting project...");

            match projects.delete(&project_name) {
                Ok(_) => {
                    spinner.finish_and_clear();
                    Message::done("The project has been deleted.");
                }
                Err(_) => {
                    spinner.finish_and_clear();
                    bail!("Failed to remove project directory because of the file system error.");
                }
            }
        }
        Some(("templates", sub)) => match sub.subcommand() {
            Some(("new", _sub)) => {
                let mut storage: Storage = Storage::load_storage()?;
                let name = Dialog::ask_string("How do you want to name this template?");
                if name.is_empty() {
                    bail!("Incorrect name for a template.");
                }

                let mut commands: Vec<String> = Vec::new();
                loop {
                    let command = Dialog::ask_string(
                        "Enter a command (or just press enter to stop entering commands):",
                    );
                    if command.trim().is_empty() {
                        break;
                    } else {
                        commands.push(command.trim().to_string());
                    }
                }

                if commands.is_empty() {
                    bail!("No commands entered.");
                }

                Message::busy("Creating template...");
                storage.add_template(&name, commands).unwrap();
                if storage.save_storage().is_ok() {
                    Message::done("Template created.");
                } else {
                    bail!("Failed to save templates.");
                }
            }
            Some(("list", _sub)) => {
                let storage: Storage = Storage::load_storage()?;
                if storage.is_templates_empty() {
                    Message::info("No templates found.");
                    return Ok(());
                }

                Message::title("Templates:");
                for template in storage.get_templates_names().iter() {
                    Message::item(template);
                }
            }
            Some(("info", sub)) => {
                let storage: Storage = Storage::load_storage()?;

                match storage.get_template(sub.get_one::<String>("name").unwrap()) {
                    Ok(template) => {
                        Message::title("Commands of this template:");
                        for command in template.iter() {
                            Message::item(command);
                        }
                    }
                    Err(_) => {
                        bail!("Template not found.");
                    }
                }
            }
            Some(("clear", _sub)) => {
                if Dialog::ask("Do you really want to clear all templates?", false) {
                    let mut storage: Storage = Storage::load_storage()?;
                    storage.clear_templates();
                    if storage.save_storage().is_ok() {
                        Message::done("All templates have been cleared.");
                    } else {
                        bail!("Failed to save templates.");
                    }
                } else {
                    Message::info("Aborted.");
                }
            }
            Some(("remove", sub)) => {
                let mut storage: Storage = Storage::load_storage()?;
                match storage.remove_template(sub.get_one::<String>("name").unwrap()) {
                    Ok(_) => {
                        if storage.save_storage().is_ok() {
                            Message::done("Template removed.");
                        } else {
                            bail!("Failed to save templates.");
                        }
                    }
                    Err(_) => bail!("Template not found."),
                }
            }
            _ => {
                bail!("Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands.");
            }
        },
        Some(("config", sub)) => match sub.subcommand() {
            Some(("path", _sub)) => {
                Message::info(Platform::get_config_path().to_str().unwrap());
            }
            Some(("edit", _sub)) => {
                let config: Config = Config::load()?;
                let editor = &config.editor.program;
                if editor.is_empty() {
                    bail!("Editor program name is not set in the configuration file.");
                }

                let path = Platform::get_config_path();
                let mut editor_args = config.editor.args;
                editor_args.push(path.to_str().unwrap().to_string());
                Program::launch_program(editor.as_str(), editor_args, "", false)?
            }
            Some(("reset", _sub)) => {
                let mut config: Config = Config::load()?;
                if Dialog::ask(
                    "Do you really want to reset your current configuration?",
                    false,
                ) {
                    config.reset();
                    config.save()?;
                    Message::done("The configuration has been reset.");
                } else {
                    Message::info("Aborted.");
                }
            }
            _ => {
                bail!("Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands.");
            }
        },
        _ => bail!("This command is not implemented."),
    }
    Ok(())
}
