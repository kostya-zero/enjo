use clap::{value_parser, Arg, Command};

pub fn get_args() -> Command {
    Command::new("enjo")
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
            Command::new("open").about("Open project in editor.").arg(
                Arg::new("name")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .num_args(1),
            ),
            Command::new("shell")
                .about("Start new shell instance in project directory.")
                .arg(
                    Arg::new("name")
                        .value_parser(value_parser!(String))
                        .required(false)
                        .num_args(1),
                ),
            Command::new("list").about("List projects."),
            Command::new("delete").about("Delete project.").arg(
                Arg::new("name")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .num_args(1),
            ),
            Command::new("path").about("Set path to projects"),
            Command::new("config").about("Open configuration file.")
        ])
}
