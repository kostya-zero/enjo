use std::process::exit;

use crate::library::Library;
use crate::platform::Platform;
use crate::program::Program;
use crate::terminal::{Dialog, Message};
use crate::{config::Config, storage::Storage};
use anyhow::{anyhow, Error, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum CompletionResult {
    Found,
    FoundSimilar(String),
    Nothing,
}

pub struct Utils;
impl Utils {
    pub fn write_config(config: Config) {
        Config::write(config).unwrap_or_else(|err| Message::fail(&format!("{}", err)));
    }

    pub fn load_projects(path: &str, display_hidden: bool) -> Library {
        Library::new(path, display_hidden).unwrap_or_else(|err| {
            Message::fail(&format!("{}", err));
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

    pub fn autocomplete(word: &str, words_list: Vec<String>) -> Option<String> {
        let suggested = Self::suggest_completion(word, words_list.clone());

        match suggested {
            CompletionResult::Found => Some(word.to_string()),
            CompletionResult::FoundSimilar(name) => {
                let answer = Dialog::ask(format!("Did you mean '{}'?", name).as_str(), true);
                if answer {
                    Some(name)
                } else {
                    None
                }
            }
            CompletionResult::Nothing => None,
        }
    }

    pub fn suggest_completion(word: &str, words_list: Vec<String>) -> CompletionResult {
        let mut found = false;
        let mut similar = false;
        let mut similar_word = String::new();

        // Searching if the same word exists in list.
        for entry in words_list.iter() {
            if found {
                break;
            }

            if *entry == word {
                found = true;
            }
        }

        if !found {
            // Searching for similar word.
            for entry in words_list.iter() {
                if similar {
                    break;
                }

                if entry.starts_with(word) {
                    similar = true;
                    similar_word.push_str(entry);
                }
            }
        }

        if found {
            CompletionResult::Found
        } else if similar {
            CompletionResult::FoundSimilar(similar_word)
        } else {
            CompletionResult::Nothing
        }
    }

    pub fn check_env() -> Result<(), Error> {
        if !Platform::check_config_exists() {
            let default_config: Config = Config::default();
            match Config::write(default_config) {
                Ok(_) => {}
                Err(e) => return Err(anyhow!(e.to_string())),
            }
        }

        if !Platform::check_templates_exists() {
            let templates = Storage::new();
            if templates.save_storage().is_err() {
                return Err(anyhow!("Failed to generate storage file."));
            }
        }

        Ok(())
    }
}
