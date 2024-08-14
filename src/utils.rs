use std::process::exit;

use crate::config::Config;
use crate::library::Library;
use crate::program::Program;
use crate::term::Term;

pub struct Utils;
impl Utils {
    pub fn get_config() -> Config {
        Config::load().unwrap_or_else(|err| {
            Term::fail(&format!("{err}"));
            exit(1);
        })
    }

    pub fn write_config(config: Config) {
        Config::write(config).unwrap_or_else(|err| Term::fail(&format!("{err}")));
    }

    pub fn load_projects(path: &str, display_hidden: bool) -> Library {
        Library::new(path, display_hidden).unwrap_or_else(|err| {
            Term::fail(&format!("{err}"));
            exit(1);
        })
    }

    pub fn launch_program(program: &str, args: Vec<String>, cwd: &str, fork_mode: bool) {
        let mut proc = Program::new(program);
        if !cwd.is_empty() {
            proc.set_cwd(cwd);
        }
        proc.set_fork_mode(fork_mode);
        proc.set_args(args.iter().map(|i| i.as_str()).collect());
        if let Err(e) = proc.run() {
            Term::fail(e.to_string().as_str());
        }
    }
}
