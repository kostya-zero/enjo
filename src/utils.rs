use std::process::exit;

use crate::config::Config;
use crate::library::Library;
use crate::program::Program;
use crate::term::Message;

pub struct Utils;
impl Utils {
    pub fn write_config(config: Config) {
        Config::write(config).unwrap_or_else(|err| Message::fail(&format!("{err}")));
    }

    pub fn load_projects(path: &str, display_hidden: bool) -> Library {
        Library::new(path, display_hidden).unwrap_or_else(|err| {
            Message::fail(&format!("{err}"));
            exit(1);
        })
    }

    pub fn launch_program(program: &str, args: Vec<String>, cwd: &str, fork_mode: bool) {
        let mut proc = Program::new(program);
        if !cwd.is_empty() {
            proc.set_cwd(cwd);
        }
        proc.set_fork_mode(fork_mode);
        proc.set_args(args);
        if let Err(e) = proc.run() {
            Message::fail(e.to_string().as_str());
        }
    }

    pub fn get_reposiotry_name_from_url(url: &str) -> Option<&str> {
        if let Some(pos) = url.rfind('/') {
            let mut filename = &url[pos + 1..];

            if filename.ends_with(".git") {
                filename = &filename[..filename.len() - 4];
            }

            Some(filename)
        } else {
            None
        }
    }
}
