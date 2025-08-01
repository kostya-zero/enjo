use std::{
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow, ensure};
use colored::Colorize;

use crate::{
    autocomplete,
    cli::{CloneArgs, ListArgs, NewArgs, OpenArgs, RemoveArgs, RenameArgs},
    config::Config,
    library::{CloneOptions, Library},
    program::launch_program,
    templates::Templates,
    terminal::{
        ask_dialog, generate_progress, print_done, print_error, print_progress, print_title,
    },
};

pub mod config;
pub mod templates;

fn resolve_project_name(project_name: &str, config: &Config, projects: &Library) -> Option<String> {
    if project_name == "-" && config.recent.enabled {
        Some(config.recent.recent_project.clone())
    } else if config.autocomplete.enabled {
        autocomplete::autocomplete(project_name, projects.get_names())
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
        let templates = Templates::load()?;
        let template = templates
            .get_template(&template_name)
            .ok_or_else(|| anyhow!("Template '{}' not found.", template_name))?;
        let started_time = Instant::now();

        println!("Generating project from template...");

        let program = &config.shell.program;
        ensure!(
            !program.is_empty(),
            "Shell is not configured in the configuration file."
        );
        let project_path = Path::new(&config.options.projects_directory)
            .join(&name)
            .to_string_lossy()
            .to_string();
        let total_commands = template.len() as i8;

        for (idx, command) in template.iter().enumerate() {
            let current = idx as i8 + 1;
            print_progress(command, current, total_commands);

            let mut args_vec = config.shell.args.clone();
            args_vec.push(command.clone());

            if let Err(e) = launch_program(
                program,
                &args_vec,
                Some(project_path.as_str()),
                false,
                args.quiet,
            ) {
                print_error("Failed to apply template. Cleaning up...");
                projects
                    .delete(&name)
                    .map_err(|err| anyhow!("Additionally, cleanup failed: {}", err.to_string()))?;
                return Err(anyhow!("Template command '{}' failed: {}", command, e));
            }
        }

        let elapsed_time = started_time.elapsed().as_millis();
        print_done(&format!("Generated in {elapsed_time} ms."));
    } else {
        print_done("Created.");
    }

    Ok(())
}

pub fn handle_clone(args: CloneArgs, config: &Config) -> Result<()> {
    let remote = args
        .remote
        .ok_or_else(|| anyhow!("You need to provide a remote URL."))?;

    let clone_options = CloneOptions {
        remote,
        name: args.name,
        branch: args.branch,
    };

    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;

    projects
        .clone(&clone_options)
        .map_err(|e| anyhow!(e.to_string()))?;

    print_done("Cloned.");
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
        print_done("Editor launched.");
        return Ok(());
    }

    Ok(())
}

pub fn handle_list(args: ListArgs, config: &Config) -> Result<()> {
    let projects = Library::new(
        &config.options.projects_directory,
        config.options.display_hidden,
    )?;
    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    let recent = &config.recent.recent_project;

    if !args.pure {
        print_title("Your projects:");
    }
    for project in projects.get_vec().iter() {
        if args.pure {
            println!("{}", project.get_name());
        } else {
            println!(
                " {} {}",
                project.get_name(),
                if project.get_name() == recent {
                    "(recent)".white().bold()
                } else {
                    "".dimmed()
                }
            );
        }
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
    print_done("Renamed.");
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

    let project_name = resolve_project_name(&name, config, &projects)
        .ok_or_else(|| anyhow!("Project not found."))?;

    let project = projects
        .get(&project_name)
        .map_err(|_| anyhow!("Project not found."))?;

    if !project.is_empty()
        && !args.force
        && !ask_dialog("The project is not empty. Continue?", false)
    {
        print_done("Aborting.");
        return Ok(());
    }

    let spinner = generate_progress().with_message("Removing project...");

    spinner.enable_steady_tick(Duration::from_millis(100));
    projects.delete(&project_name)?;
    spinner.finish_and_clear();

    print_done("Removed.");
    Ok(())
}

const ENJO_ZEN: [&str; 10] = [
    "Projects should be simple.",
    "Each command does one thing well.",
    "Configuration is explicit.",
    "Sensible defaults guide the way.",
    "The shell is a friend.",
    "Templates accelerate your workflow.",
    "Cross-platform by design.",
    "Clear messages beat surprises.",
    "Your editor is respected.",
    "Enjoy your work.",
];

pub fn handle_zen() -> Result<()> {
    for line in ENJO_ZEN.iter() {
        println!(" {} {line}", "*".dimmed());
    }
    Ok(())
}
