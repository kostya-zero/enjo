use std::time::Instant;

use anyhow::{Result, anyhow, ensure};
use colored::Colorize;

use crate::{
    cli::{CloneArgs, NewArgs, OpenArgs, RemoveArgs, RenameArgs},
    config::Config,
    library::{CloneOptions, Library},
    program::launch_program,
    terminal::{Dialog, Message},
    utils,
};

pub mod config;
pub mod templates;

fn resolve_project_name(project_name: &str, config: &Config, projects: &Library) -> Option<String> {
    if project_name == "-" && config.recent.enabled {
        Some(config.recent.recent_project.clone())
    } else if config.autocomplete.enabled {
        utils::autocomplete(project_name, projects.get_names())
    } else {
        Some(project_name.to_string())
    }
}

pub fn handle_new(args: NewArgs, config: &Config) -> Result<()> {
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let name = args
        .name
        .ok_or_else(|| anyhow!("Provide a name for a new project."))?;

    projects.create(&name)?;

    if let Some(template_name) = args.template {
        let started_time = Instant::now();

        let result = utils::apply_template(&template_name, config, &name, args.quiet);
        if result.is_err() {
            Message::error("Failed to apply template. Cleaning up...");
            projects
                .delete(&name)
                .map_err(|e| anyhow!("Additionally, cleanup failed: {}", e.to_string()))?;
        }
        let elapsed_time = started_time.elapsed().as_millis();
        println!("Generated project from template in {elapsed_time} ms.");
    }

    println!("Done.");
    Ok(())
}

pub fn handle_clone(args: CloneArgs, config: &Config) -> Result<()> {
    let remote = args
        .remote
        .ok_or_else(|| anyhow!("You need to provide a remote URL."))?;

    let branch = args.branch;
    let name = args.name;

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

    println!("The project has been cloned.");
    Ok(())
}

pub fn handle_open(args: OpenArgs, config: &mut Config) -> Result<()> {
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let project_name = args
        .name
        .ok_or_else(|| anyhow!("The project name is not provided."))?;

    let name = resolve_project_name(&project_name, config, &projects)
        .ok_or_else(|| anyhow!("Project not found."))?;

    let project = projects
        .get(&name)
        .map_err(|_| anyhow!("Project not found."))?;

    let (program, launch_args, fork_mode) = if args.shell {
        (&config.shell.program, Vec::<String>::new(), false)
    } else {
        (
            &config.editor.program,
            config.editor.args.clone(),
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

    if args.shell {
        println!(
            "{}",
            "======== SHELL SESSION STARTED ========".bold().white()
        );
    }

    launch_program(
        program,
        &launch_args,
        Some(project.get_path()),
        fork_mode,
        false,
    )?;

    if args.shell {
        println!(
            "{}",
            "========  SHELL SESSION ENDED  ========".bold().white()
        );
    }

    if fork_mode {
        // Because only editor could be launched in fork mode.
        println!("Editor launched.");
        return Ok(());
    }

    Ok(())
}

pub fn handle_list(config: &Config) -> Result<()> {
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;
    if projects.is_empty() {
        println!("No projects found.");
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

    Ok(())
}

pub fn handle_rename(args: RenameArgs, config: &Config) -> Result<()> {
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let old_name = args
        .old_name
        .ok_or_else(|| anyhow!("No project to rename."))?;
    let new_name = args
        .new_name
        .ok_or_else(|| anyhow!("Provide a new name for a project."))?;

    projects.rename(&old_name, &new_name)?;
    println!("Done.");
    Ok(())
}

pub fn handle_remove(args: RemoveArgs, config: &Config) -> Result<()> {
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    let name = args
        .name
        .ok_or_else(|| anyhow!("Provide a name of project to remove."))?;

    let project = projects
        .get(&name)
        .map_err(|_| anyhow!("Project not found."))?;

    if !project.is_empty()
        && !args.force
        && !Dialog::ask("The project is not empty. Continue?", false)
    {
        println!("Aborting.");
        return Ok(());
    }

    println!("Removing project...");
    projects.delete(&name)?;

    println!("The project has been removed.");
    Ok(())
}
