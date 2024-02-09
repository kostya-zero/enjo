use clap::{Arg, ArgAction, Command, value_parser};

pub fn get_args() -> Command {
    Command::new("enjo")
        .name("enjo")
        .about("Minimalist workspace assistant tool.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("new")
                .about("Create new project")
                .args([Arg::new("name")
                    .default_value("")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .num_args(1)]),
            Command::new("open").about("Open project in editor.").args([
                Arg::new("name")
                    .help("Name of the project to open.")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .default_value("")
                    .num_args(1),
                Arg::new("append")
                    .help("Append path to the project.")
                    .long("append")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .default_value("")
                    .num_args(1),
                Arg::new("shell")
                    .help("Open shell instead of editor.")
                    .long("shell")
                    .action(ArgAction::SetTrue),
            ]),
            Command::new("list").about("List projects."),
            Command::new("delete").about("Delete project.").arg(
                Arg::new("name")
                    .value_parser(value_parser!(String))
                    .default_value("")
                    .required(false)
                    .num_args(1),
            ),
            Command::new("config").about("Manage your config file.")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("edit").about("Edit configuration file."),
                    Command::new("path").about("Get path to the configuration file."),
                    Command::new("reset").about("Reset your configuration.").arg(
                        Arg::new("yes")
                            .help("You understand that your configuration will be reset WITHOUT A WAY TO RESTORE.")
                            .long("yes")
                            .action(ArgAction::SetTrue)
                    )
                ]),
        ])
}
