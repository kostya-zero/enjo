use crate::cli::{Cli, Commands, ConfigCommands, TemplatesCommands};
use crate::config::Config;
use crate::handlers::{
    config, handle_clone, handle_list, handle_new, handle_open, handle_remove, handle_rename,
    templates,
};
use crate::platform::Platform;
use crate::templates::Templates;
use anyhow::{Result, anyhow};
use clap::Parser;

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

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    check_env()?;

    let mut config: Config = Config::load()?;
    let mut templates = Templates::load()?;

    match cli.cmd {
        Commands::New(args) => handle_new(args, &config),
        Commands::Clone(args) => handle_clone(args, &config),
        Commands::Open(args) => handle_open(args, &mut config),
        Commands::List => handle_list(&config),
        Commands::Rename(args) => handle_rename(args, &config),
        Commands::Remove(args) => handle_remove(args, &config),
        Commands::Templates { command } => match command {
            TemplatesCommands::New => templates::handle_new(&mut templates),
            TemplatesCommands::List => templates::handle_list(&templates),
            TemplatesCommands::Edit => templates::handle_edit(&config),
            TemplatesCommands::Info(args) => templates::handle_info(args, &templates),
            TemplatesCommands::Clear => templates::handle_clear(&mut templates),
            TemplatesCommands::Remove(args) => templates::handle_remove(args, &mut templates),
        },
        Commands::Config { command } => match command {
            ConfigCommands::Path => config::handle_path(),
            ConfigCommands::Edit => config::handle_edit(&config),
            ConfigCommands::Reset => config::handle_reset(&mut config),
        },
    }
}
