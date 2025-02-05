use clap::{value_parser, Arg, ArgAction, Command};

pub fn build_cli() -> Command {
    Command::new("enjo")
        .name("enjo")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("new").about("Create new project").args([
                Arg::new("name")
                    .help("Name for a new project.")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .num_args(1),
                Arg::new("template")
                    .help("Template to use.")
                    .value_parser(value_parser!(String))
                    .short('t')
                    .long("template")
                    .num_args(1),
                Arg::new("quite")
                    .help("Do not show the output of the running commands.")
                    .long("quite")
                    .action(ArgAction::SetTrue),
            ]),
            Command::new("clone")
                .about("Clone Git repository (requires git to be installed).")
                .args([
                    Arg::new("remote")
                        .help("Repository to be cloned.")
                        .value_parser(value_parser!(String))
                        .required(false)
                        .num_args(1),
                    Arg::new("name")
                        .help("Name of directory for this repo.")
                        .value_parser(value_parser!(String))
                        .short('n')
                        .long("name")
                        .required(false)
                        .num_args(1),
                    Arg::new("branch")
                        .help("Branch to use as default while cloning.")
                        .value_parser(value_parser!(String))
                        .short('b')
                        .long("branch")
                        .required(false)
                        .num_args(1),
                ]),
            Command::new("open")
                .about("Open project in editor or shell.")
                .args([
                    Arg::new("name")
                        .help("Name of the project to open.")
                        .value_parser(value_parser!(String))
                        .required(false)
                        .num_args(1),
                    Arg::new("shell")
                        .help("Open shell instead of editor.")
                        .long("shell")
                        .short('s')
                        .action(ArgAction::SetTrue),
                ]),
            Command::new("list").about("List available projects."),
            Command::new("rename").about("Rename project.").args([
                Arg::new("name")
                    .help("Name of the project to rename")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .num_args(1),
                Arg::new("newname")
                    .help("New name.")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .num_args(1),
            ]),
            Command::new("delete").about("Delete project.").arg(
                Arg::new("name")
                    .help("Name of the project to delete.")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .num_args(1),
            ),
            Command::new("templates")
                .about("Manage your templates.")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("new").about("Create new template."),
                    Command::new("list").about("List available templates."),
                    Command::new("info")
                        .about("View information about template.")
                        .arg(
                            Arg::new("name")
                                .help("Name of the template.")
                                .value_parser(value_parser!(String))
                                .required(false)
                                .num_args(1),
                        ),
                    Command::new("remove").about("Remove template.").arg(
                        Arg::new("name")
                            .help("Name of the template to remove.")
                            .value_parser(value_parser!(String))
                            .required(false)
                            .num_args(1),
                    ),
                ]),
            Command::new("config")
                .about("Manage your configuration.")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("edit").about("Edit configuration file."),
                    Command::new("path").about("Get path to the configuration file."),
                    Command::new("reset").about("Reset your configuration."),
                ]),
        ])
}
