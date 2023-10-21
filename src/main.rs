use std::{fs, path::Path};

use args::get_args;
use config::Config;
use manager::Manager;
use proc::Proc;
use term::Term;

mod args;
mod config;
mod manager;
mod proc;
mod term;

fn main() {
    if !Manager::check_exists() {
        let default_config: Config = Manager::make_default();
        Manager::write_config(default_config).expect("Failed to write config.");
    }

    let args = get_args().get_matches();
    match args.subcommand() {
        Some(("new", sub)) => {
            let config: Config = Manager::load_config().ok().unwrap();
            let mut path: String = config.get_path().unwrap();
            if !Path::new(&path).exists() {
                Term::fail("Directory with projects by given path in config not found.");
            }
            let name = sub.get_one::<String>("name").unwrap();
            if name.is_empty() {
                Term::fail("Cant create project with empty name.");
            }
            path.push_str(format!("/{}", name).as_str());

            if Path::new(&path).exists() {
                Term::fail("Project with same name already exists.");
            }

            if fs::create_dir(&path).is_err() {
                Term::fail("Failed to create directory for new project.");
            }

            Term::done("Project created.");
        }
        Some(("open", sub)) => {
            let config: Config = Manager::load_config().expect("Faield to load config.");

            let path: String = config.get_path().unwrap();
            let editor: String = config.get_editor().unwrap();

            if path.is_empty() {
                Term::fail("Path option is empty!");
            }

            if editor.is_empty() {
                Term::fail("Editor option is empty!");
            }

            if !Path::new(&path).exists() {
                Term::fail("Directory with project not found.");
            }

            let project: &str = sub.get_one::<String>("name").unwrap();
            let fullpath = path.clone() + "/" + project;
            if !Path::new(&fullpath).exists() {
                Term::fail("Project not found.");
            }

            let mut proc: Proc = Proc::new(editor.as_str());
            proc.set_args(config.get_editor_args().unwrap());
            proc.set_cwd(fullpath.as_str());
            proc.run();
        }
        Some(("list", _sub)) => {
            let config: Config = Manager::load_config().expect("Faield to load config.");
            let path: String = config.get_path().unwrap();

            if !Path::new(&path).exists() {
                Term::fail("Directory with projects by given path in config not found.");
            }

            let mut projects: Vec<String> = Vec::new();
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if entry.metadata().map(|m| m.is_dir()).unwrap_or(false)
                            && !name.starts_with('.')
                        {
                            projects.push(name.to_owned());
                        }
                    }
                }
            }

            Term::list_title("All projects");
            for i in projects {
                Term::item(i.as_str());
            }
        }
        Some(("delete", sub)) => {
            let config: Config = Manager::load_config().expect("Faield to load config.");
            let path: String = config.get_path().unwrap();

            if !Path::new(&path).exists() {
                Term::fail("Directory with projects by given path in config not found.");
            }

            let project: &str = sub.get_one::<String>("name").unwrap();
            let fullpath = path.clone() + "/" + project;
            if !Path::new(&fullpath).exists() {
                Term::fail("Project not found.");
            }

            fs::remove_dir_all(fullpath)
                .expect("Failed to remove project directory due to lack of permissions.");
            Term::done("Project deleted.")
        }
        Some(("config", _sub)) => {
            let config: Config = Manager::load_config().expect("Faield to load config.");
            let editor: String = config.get_editor().unwrap();

            if editor.is_empty() {
                Term::fail("Editor option is empty!");
            }

            let mut proc: Proc = Proc::new(editor.as_str());
            let mut editor_args = config.get_editor_args().unwrap();
            let config_path = Manager::get_config_path();
            editor_args.push(config_path.as_str());
            proc.set_args(editor_args);
            proc.run();
        }
        _ => Term::error("Command not found or it's not implemented yet."),
    }
}
