use std::process::exit;

use anyhow::{Result, anyhow};
use clap::Parser;
use enjo::{
    cli::{Cli, Commands, ConfigCommands, TemplatesCommands},
    commands::{
        config, handle_clone, handle_list, handle_new, handle_open, handle_remove, handle_rename,
        handle_zen, templates,
    },
    config::Config,
    platform::Platform,
    templates::Templates,
    terminal::print_error,
};

fn check_env() -> Result<()> {
    if !Platform::check_config_exists() {
        let default_config: Config = Config::default();
        default_config
            .save(Platform::get_config_path())
            .map_err(|e| anyhow!(e.to_string()))?;
    }

    if !Platform::check_templates_exists() {
        let templates = Templates::new();
        templates
            .save(&Platform::get_templates_path())
            .map_err(|e| anyhow!(e.to_string()))?;
    }

    Ok(())
}

fn print_version() {
    let mode = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    println!("enjo {} {mode}", env!("CARGO_PKG_VERSION"));
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        print_version();
        return;
    }

    if let Err(e) = check_env() {
        print_error(&e.to_string());
        exit(1);
    }

    if cli.cmd.is_none() {
        println!("Nothing to do. Use `enjo --help` to see available commands.");
        return;
    }

    let result = match cli.cmd.unwrap() {
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
    };

    if let Err(e) = result {
        print_error(&e.to_string());
        exit(1);
    }
}
