use clap::{value_parser, Arg, ArgAction, Command};

pub fn get_args() -> Command {
    Command::new("vel")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("init").about("Initialize new project").args([
                Arg::new("name")
                    .default_value("")
                    .value_parser(value_parser!(String))
                    .required(false)
                    .num_args(1),
                Arg::new("command")
                    .long("command")
                    .short('c')
                    .default_value("")
                    .value_parser(value_parser!(String))
                    .num_args(1)
                    .required(false),
            ]),
            Command::new("list").about("List projects."),
            Command::new("path").about("Set path to projects")
        ])
}
