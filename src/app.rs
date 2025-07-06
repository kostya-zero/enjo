use crate::args::build_cli;
use crate::config::Config;
use crate::library::{CloneOptions, Library};
use crate::platform::Platform;
use crate::program::launch_program;
use crate::templates::Templates;
use crate::terminal::{Dialog, Message};
use crate::utils;
use anyhow::{Result, anyhow, bail, ensure};
use colored::Colorize;
use std::time::Instant;

fn check_env() -> Result<()> {
    if !Platform::check_config_exists() {
        let default_config: Config = Config::default();
        default_config.save().map_err(|e| anyhow!(e.to_string()))?;
    }

    if !Platform::check_templates_exists() {
        let templates = Templates::new();
        templates.save().map_err(|e| anyhow!(e.to_string()))?;
    }

    Ok(())
}

fn resolve_project_name(project_name: &str, config: &Config, projects: &Library) -> Option<String> {
    if project_name == "-" && config.recent.enabled {
        Some(config.recent.recent_project.clone())
    } else if config.autocomplete.enabled {
        utils::autocomplete(project_name, projects.get_names())
    } else {
        Some(project_name.to_string())
    }
}

pub fn run() -> Result<()> {
    let args = build_cli().get_matches();

    check_env()?;

    let mut config: Config = Config::load()?;
    let mut templates = Templates::load()?;

    match args.subcommand() {
        Some(("new", sub)) => {
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;

            let name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("Provide a name for a new project."))?;

            projects.create(name)?;

            if let Some(template_name) = sub.get_one::<String>("template") {
                let started_time = Instant::now();

                let result =
                    utils::apply_template(template_name, &config, name, sub.get_flag("quiet"));
                if result.is_err() {
                    Message::error("Failed to apply template. Cleaning up...");
                    projects
                        .delete(name)
                        .map_err(|e| anyhow!("Additionally, cleanup failed: {}", e.to_string()))?;
                }
                let elapsed_time = started_time.elapsed().as_millis();
                Message::print(&format!(
                    "Generated project from template in {elapsed_time} ms."
                ));
            }

            Message::print("Done.")
        }
        Some(("clone", sub)) => {
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

            projects
                .clone(&clone_options)
                .map_err(|e| anyhow!(e.to_string()))?;

            Message::print("The project has been cloned.");
        }
        Some(("open", sub)) => {
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;

            let project_name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("The project name is not provided."))?;

            let name = resolve_project_name(project_name, &config, &projects).ok_or_else(|| anyhow!("Project not found."))?;

            let project = projects
                .get(&name)
                .map_err(|_| anyhow!("Project not found."))?;

            let (program, args, end_message, start_message, fork_mode) = if sub.get_flag("shell") {
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
            launch_program(program, &args, Some(project.get_path()), fork_mode, false)?;

            if fork_mode {
                // Because only editor could be launched in fork mode.
                Message::print("Editor launched.");
                return Ok(());
            }

            Message::print(end_message);
        }
        Some(("list", _sub)) => {
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
                Message::item(&format!(
                    "{} {}",
                    project.get_name(),
                    if project.get_name() == recent {
                        "(recent)".white().bold()
                    } else {
                        "".dimmed()
                    }
                ));
            }
        }
        Some(("rename", sub)) => {
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;

            let args_name = sub
                .get_one::<String>("old")
                .ok_or_else(|| anyhow!("No project to rename."))?;

            let new_name = sub
                .get_one::<String>("new")
                .ok_or_else(|| anyhow!("Provide a new name for a project."))?;

            projects.rename(args_name, new_name)?;
            Message::print("Done.");
        }
        Some(("remove", sub)) => {
            let projects = Library::new(
                &config.options.projects_directory,
                config.options.display_hidden,
            )?;

            let args_name = sub
                .get_one::<String>("name")
                .ok_or_else(|| anyhow!("Provide a name of project to remove."))?;

            let project = projects
                .get(args_name)
                .map_err(|_| anyhow!("Project not found."))?;

            if !project.is_empty()
                && !sub.get_flag("force")
                && !Dialog::ask("The project is not empty. Continue?", false)
            {
                Message::print("Aborting.");
                return Ok(());
            }

            Message::print("Removing project...");
            projects.delete(args_name)?;

            Message::print("The project has been removed.");
        }
        Some(("templates", sub)) => match sub.subcommand() {
            Some(("new", _sub)) => {
                let name = Dialog::ask_string("Name of new template?");
                if name.is_empty() {
                    bail!("Incorrect name for a template.");
                }

                let mut commands: Vec<String> = Vec::new();
                loop {
                    let command = Dialog::ask_string("Enter a command (press enter to finish):");
                    if command.is_empty() {
                        break;
                    }
                    commands.push(command.to_string());
                }

                ensure!(!commands.is_empty(), "No commands entered.");

                Message::print("Creating template...");
                templates.add_template(&name, commands)?;
                if templates.save().is_ok() {
                    Message::print("Template created.");
                } else {
                    bail!("Failed to save templates.");
                }
            }
            Some(("list", _sub)) => {
                if templates.is_empty() {
                    Message::print("No templates found.");
                    return Ok(());
                }

                Message::title("Templates:");
                for template in templates.list_templates().iter() {
                    Message::item(template);
                }
            }
            Some(("edit", _sub)) => {
                let editor = &config.editor.program;
                if editor.is_empty() {
                    bail!("Editor program name is not set in the configuration file.");
                }

                let path = Platform::get_templates_path();
                let mut editor_args = config.editor.args;
                editor_args.push(path.to_str().unwrap().to_string());
                launch_program(editor, &editor_args, None, false, false)?
            }
            Some(("info", sub)) => {
                let name = sub
                    .get_one::<String>("name")
                    .ok_or_else(|| anyhow!("Provide a name of the template."))?;

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
                if Dialog::ask("Clear all templates?", false) {
                    templates.clear();
                    templates.save()?;
                    Message::print("All templates have been cleared.");
                } else {
                    Message::print("Aborted.");
                }
            }
            Some(("remove", sub)) => {
                let name = sub
                    .get_one::<String>("name")
                    .ok_or_else(|| anyhow!("Provide a name of template to delete."))?;
                match templates.remove_template(name) {
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
                bail!("This command is not implemented.");
            }
        },
        Some(("config", sub)) => match sub.subcommand() {
            Some(("path", _sub)) => {
                Message::print(Platform::get_config_path().to_str().unwrap());
            }
            Some(("edit", _sub)) => {
                let editor = &config.editor.program;
                if editor.is_empty() {
                    bail!("Editor program name is not set in the configuration file.");
                }

                let path = Platform::get_config_path();
                let mut editor_args = config.editor.args;
                editor_args.push(path.to_str().unwrap().to_string());
                launch_program(editor, &editor_args, None, false, false)?
            }
            Some(("reset", _sub)) => {
                if Dialog::ask("Reset your current configuration?", false) {
                    config.reset();
                    config.save()?;
                    Message::print("The configuration has been reset.");
                } else {
                    Message::print("Aborted.");
                }
            }
            _ => {
                bail!("This command is not implemented.");
            }
        },
        _ => bail!("This command is not implemented."),
    }
    Ok(())
}
