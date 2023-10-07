use std::{fs, iter::repeat_with, path::Path, process::exit};

use args::get_args;
use config::Config;
use manager::Manager;
use term::Term;

mod actions;
mod args;
mod config;
mod manager;
mod term;

fn main() {
    if Manager::check_exists() {
        let default_config: Config = Manager::make_default();
        Manager::write_config(default_config).expect("Failed to write config.");
    }

    let args = get_args().get_matches();
    match args.subcommand() {
        Some(("init", sub)) => {
            let config: Config = Manager::load_config().ok().unwrap();
            let mut path: String = config.get_path().unwrap();
            if !Path::new(&path).exists() {
                Term::error("Directory with projects by given path in config not found.");
                exit(1);
            }
            let name = sub.get_one::<String>("name").unwrap();
            if name.is_empty() {
                Term::error("Cant create project with empty name.");
                exit(1);
            }
            path.push_str(format!("/{}", name).as_str());

            if Path::new(&path).exists() {
                Term::error("Project with same name already exists.");
                exit(1);
            }

            if fs::create_dir(&path).is_err() {
                Term::error("Failed to create directory for new project.");
                exit(0);
            }

            Term::done("Project created.");
        }
        _ => Term::error("Command not found or it's not implemented yet."),
    }
}
