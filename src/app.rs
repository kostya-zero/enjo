use crate::args::build_cli;
use crate::config::Config;
use crate::library::{CloneOptions, Library};
use crate::platform::Platform;
use crate::program::Program;
use crate::templates::Templates;
use crate::terminal::{Dialog, Message, create_spinner};
use crate::utils::Utils;
use anyhow::{Result, anyhow, bail, ensure};
use std::ops::Deref;
use std::time::Instant;

fn resolve_project_name(project_name: &str, config: &Config, projects: &Library) -> Result<String> {
    if project_name == "-" {
        if config.recent.recent_project.is_empty() {
            bail!("No project was opened recently.")
        }
        Ok(config.recent.recent_project.clone())
    } else if config.autocomplete.enabled {
        let name = Utils::autocomplete(project_name, projects.get_names());
        if let Some(name) = name {
            Ok(name)
        } else {
            bail!("Project not found.")
        }
    } else {
        Ok(project_name.to_string())
    }
}

pub fn run() -> Result<()> {
    let args = build_cli().get_matches();

    Utils::check_env()?;

    match args.subcommand() {
        Some(("new", sub)) => {
            let config: Config = Config::load()?;
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;

            let name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("You need to provide a name for your new project."))?;

            projects.create(name)?;

            if let Some(template_name) = sub.get_one::<String>("template") {
                let started_time = Instant::now();
                if let Err(e) = Utils::apply_template(
                    template_name,
                    name,
                    &config.options.projects_directory,
                    sub.get_flag("quiet"),
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
                let elapsed_time = started_time.elapsed().as_millis();
                Message::print(&format!(
                    "Generated project from template in {} ms.",
                    elapsed_time
                ));
            } else {
                Message::print("Done.")
            }
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

            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;
            match projects.clone(&clone_options) {
                Ok(_) => Message::print("The project has been cloned."),
                Err(e) => bail!(e.to_string()),
            }
        }
        Some(("open", sub)) => {
            let mut config: Config = Config::load()?;
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;
            let project_name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("The project name is not provided."))?;

            let name = resolve_project_name(project_name, &config, &projects)?;

            let project = projects.get(&name).map_err(|_| anyhow!("Project not found."))?;

            let (program, args, end_message, start_message, fork_mode) =
                if sub.get_flag("shell") {
                    (
                        &config.shell.program,
                        Vec::<String>::new(),
                        "Shell session ended.",
                        "Launching shell...",
                        false,
                    )
                } else {
                    (
                        &config.editor.program,
                        config.editor.args.clone(),
                        "Editor session ended.",
                        "Launching editor...",
                        config.editor.fork_mode,
                    )
                };

            ensure!(
                !program.is_empty(),
                "Required program is not specified in configuration file."
            );

            if config.recent.enabled && name != config.recent.recent_project {
                config.recent.recent_project = name.clone();
                config.save()?;
            }

            Message::print(start_message);
            Program::launch_program(program, args, project.get_path_str(), fork_mode)?;

            if fork_mode {
                // Because only editor could be launched in fork mode.
                Message::print("Editor launched.");
                return Ok(());
            }

            Message::print(end_message);
        }
        Some(("list", _sub)) => {
            let config: Config = Config::load()?;
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;
            if projects.is_empty() {
                Message::print("No projects found.");
                return Ok(());
            }

            let recent = &config.recent.recent_project;

            Message::title("Your projects:");
            for project in projects.get_vec().iter() {
                if project.get_name() == recent {
                    Message::item(&format!("{} \x1b[1m(recent)\x1b[0m", project.get_name()));
                } else {
                    Message::item(project.get_name());
                }
            }
        }
        Some(("rename", sub)) => {
            let config: Config = Config::load()?;
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;

            let args_name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("You need to provide a name of the project you want to rename."))?;

            let name = resolve_project_name(args_name, &config, &projects)?;

            let new_name = sub
                .get_one::<String>("newname")
                .ok_or_else(|| anyhow!("You need to provide a new name for the project."))?;

            match projects.rename(name.as_ref(), new_name) {
                Ok(_) => Message::print(&format!("The project was renamed to '{}'.", new_name)),
                Err(e) => bail!(e.to_string()),
            }
        }
        Some(("delete", sub)) => {
            let config: Config = Config::load()?;
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;

            let args_name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("You need to provide a name of the project you want to delete."))?;

            let project_name = resolve_project_name(args_name, &config, &projects)?;

            let project = projects
                .get(&project_name)
                .map_err(|_| anyhow!("Project not found."))?;

            let force_delete = sub.get_flag("force");
            let is_empty = project.is_empty()?;

            if !is_empty
                && !force_delete
                && !Dialog::ask("Are you sure you want to delete this project?", false)
            {
                Message::print("Aborting.");
                return Ok(());
            }

            let spinner = create_spinner();
            spinner.set_message("Deleting project...");

            match projects.delete(&project_name) {
                Ok(_) => {
                    spinner.finish_and_clear();
                    Message::print("The project has been deleted.");
                }
                Err(_) => {
                    spinner.finish_and_clear();
                    bail!("Failed to remove project directory because of the file system error.");
                }
            }
        }
        Some(("templates", sub)) => match sub.subcommand() {
            Some(("new", _sub)) => {
                let mut templates = Templates::load()?;
                let name = Dialog::ask_string("Name of new template?");
                if name.is_empty() {
                    bail!("Incorrect name for a template.");
                }

                let mut commands: Vec<String> = Vec::new();
                loop {
                    let command = Dialog::ask_string("Enter a command (or press enter to finish):");
                    if command.trim().is_empty() {
                        break;
                    } else {
                        commands.push(command.trim().to_string());
                    }
                }

                if commands.is_empty() {
                    bail!("No commands entered.");
                }

                Message::print("Creating template...");
                templates.add_template(&name, commands)?;
                if templates.save().is_ok() {
                    Message::print("Template created.");
                } else {
                    bail!("Failed to save templates.");
                }
            }
            Some(("list", _sub)) => {
                let templates = Templates::load()?;
                if templates.is_empty() {
                    Message::print("No templates found.");
                    return Ok(());
                }

                Message::title("Templates:");
                for template in templates.list_templates().iter() {
                    Message::item(template);
                }
            }
            Some(("info", sub)) => {
                let templates = Templates::load()?;

                let name = sub.get_one::<String>("name").ok_or_else(|| {
                    anyhow!("You need to provide a name of the template.")
                })?;

                match templates.get_template(name) {
                    Some(template) => {
                        Message::title("Commands of this template:");
                        for command in template.iter() {
                            Message::item(command);
                        }
                    }
                    None => {
                        bail!("Template not found.");
                    }
                }
            }
            Some(("clear", _sub)) => {
                if Dialog::ask("Do you really want to clear all templates?", false) {
                    let mut templates = Templates::load()?;
                    templates.clear();
                    if templates.save().is_ok() {
                        Message::print("All templates have been cleared.");
                    } else {
                        bail!("Failed to save templates.");
                    }
                } else {
                    Message::print("Aborted.");
                }
            }
            Some(("remove", sub)) => {
                let mut templates = Templates::load()?;
                match templates.remove_template(sub.get_one::<String>("name").unwrap()) {
                    Ok(_) => {
                        if templates.save().is_ok() {
                            Message::print("Template removed.");
                        } else {
                            bail!("Failed to save templates.");
                        }
                    }
                    Err(_) => bail!("Template not found."),
                }
            }
            _ => {
                bail!(
                    "Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands."
                );
            }
        },
        Some(("config", sub)) => match sub.subcommand() {
            Some(("path", _sub)) => {
                Message::print(Platform::get_config_path().to_str().unwrap());
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
                if Dialog::ask("Reset your current configuration?", false) {
                    config.reset();
                    config.save()?;
                    Message::print("The configuration has been reset.");
                } else {
                    Message::print("Aborted.");
                }
            }
            _ => {
                bail!(
                    "Unknown or not specified subcommand. Use `enjo config --help` to get list of all subcommands."
                );
            }
        },
        _ => bail!("This command is not implemented."),
    }
    Ok(())
}
