use crate::cli::{Cli, Commands, ConfigCommands, TemplatesCommands};
use crate::config::Config;
use crate::handlers::{
    config, handle_clone, handle_list, handle_new, handle_open, handle_remove, handle_rename,
    handle_zen, templates,
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

    match cli.cmd {
        Commands::New(args) => handle_new(args),
        Commands::Clone(args) => handle_clone(args),
        Commands::Open(args) => handle_open(args),
        Commands::List(args) => handle_list(args),
        Commands::Rename(args) => handle_rename(args),
        Commands::Remove(args) => handle_remove(args),
        Commands::Templates { command } => match command {
            TemplatesCommands::New => templates::handle_new(),
            TemplatesCommands::List(args) => templates::handle_list(args),
            TemplatesCommands::Edit => templates::handle_edit(),
            TemplatesCommands::Info(args) => templates::handle_info(args),
            TemplatesCommands::Clear => templates::handle_clear(),
            TemplatesCommands::Remove(args) => templates::handle_remove(args),
        },
        Commands::Config { command } => match command {
            ConfigCommands::Path => config::handle_path(),
            ConfigCommands::Edit => config::handle_edit(),
            ConfigCommands::Reset => config::handle_reset(),
        },
        Commands::Zen => handle_zen(),
    }
}
