﻿use clap::{ArgAction, Args, Parser, Subcommand};
use std::env;

/// Yet another manager for your projects.
#[derive(Parser)]
#[command(
    name = "kanri",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    subcommand_required = false, 
    arg_required_else_help = false,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Commands>,

    /// Print the version of Kanri.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub version: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create new project.
    New(NewArgs),

    /// Clone Git repository (requires git to be installed).
    Clone(CloneArgs),

    /// Open project in editor or shell [alias: o]
    #[command(alias = "o")]
    Open(OpenArgs),

    /// List available projects [alias: ls]
    #[command(alias = "ls")]
    List(ListArgs),

    /// Rename project.
    Rename(RenameArgs),

    /// Remove project [alias: rm]
    #[command(alias = "rm")]
    Remove(RemoveArgs),

    /// Manage your templates.
    Templates {
        #[command(subcommand)]
        command: TemplatesCommands,
    },

    /// Manage your configuraticlion.
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Display the Zen of Kanri.
    Zen,
}

#[derive(Args)]
pub struct NewArgs {
    /// Name for a new project.
    pub name: Option<String>,

    /// Template to use for a new project.
    #[arg(short, long)]
    pub template: Option<String>,

    /// Hide the output of running commands.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub quiet: bool,
}

#[derive(Args)]
pub struct CloneArgs {
    /// URL of repository to clone.
    pub remote: Option<String>,

    /// Directory name for the cloned repository.
    #[arg(short, long)]
    pub name: Option<String>,

    /// Branch to clone.
    #[arg(short, long)]
    pub branch: Option<String>,
}

#[derive(Args)]
pub struct OpenArgs {
    /// Name of the project to open.
    pub name: Option<String>,

    /// Open shell in this project.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub shell: bool,
}

#[derive(Args)]
pub struct ListArgs {
    /// Display list without styling
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub pure: bool,
}

#[derive(Args)]
pub struct RenameArgs {
    /// Old project name.
    pub old_name: Option<String>,

    /// New project name.
    pub new_name: Option<String>,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Name of the project to remove.
    pub name: Option<String>,

    /// Force remove without confirmation
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub force: bool,
}

#[derive(Subcommand)]
pub enum TemplatesCommands {
    /// Create new template.
    New,

    /// List available templates.
    List(TemplatesListArgs),

    /// Edit templates.
    Edit,

    /// Clear all templates.
    Clear,

    // Prints the path to the file with templates.
    Path,

    /// View information about template.
    Info(TemplatesInfoArgs),

    /// Remove template.
    Remove(TemplatesRemoveArgs),
}

#[derive(Args)]
pub struct TemplatesListArgs {
    /// Display list without styling
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub pure: bool,
}

#[derive(Args)]
pub struct TemplatesInfoArgs {
    /// Name of the template.
    pub name: Option<String>,

    /// Display list without styling
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub pure: bool,
}

#[derive(Args)]
pub struct TemplatesRemoveArgs {
    /// Name of the template to remove.
    pub name: Option<String>,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Edit configuration file.
    Edit,

    /// Get path to the configuration file.
    Path,

    /// Reset your configuration.
    Reset,
}
