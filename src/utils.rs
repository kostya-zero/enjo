use crate::config::Config;
use crate::platform::{Platform, PlatformName};
use crate::program::Program;
use crate::templates::Templates;
use crate::terminal::{Dialog, Message};
use anyhow::{Error, Result, anyhow, bail};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub enum CompletionResult {
    Found,
    FoundSimilar(String),
    Nothing,
}

pub struct Utils;
impl Utils {
    pub fn autocomplete(word: &str, words_list: Vec<&str>) -> Option<String> {
        let suggested = Self::suggest_completion(word, words_list.clone());

        match suggested {
            CompletionResult::Found => Some(word.to_string()),
            CompletionResult::FoundSimilar(name) => {
                let answer = Dialog::ask(format!("Did you mean '{}'?", name).as_str(), true);
                if answer { Some(name) } else { None }
            }
            CompletionResult::Nothing => None,
        }
    }

    pub fn suggest_completion(word: &str, words_list: Vec<&str>) -> CompletionResult {
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
            match default_config.save() {
                Ok(_) => {}
                Err(e) => bail!(e.to_string()),
            }
        }

        if !Platform::check_templates_exists() {
            let templates = Templates::new();
            if templates.save().is_err() {
                bail!("Failed to generate templates file.");
            }
        }

        Ok(())
    }

    pub fn apply_template(
        template_name: &str,
        project_name: &str,
        base_path: &str,
        quite: bool,
    ) -> Result<(), Error> {
        let templates = Templates::load().map_err(|e| anyhow!("Failed to load templates: {}", e))?;
        let template = templates
            .get_template(template_name)
            .ok_or_else(|| anyhow!("Template '{}' not found", template_name))?;

        Message::print("Generating project from template...");

        let program = match Platform::get_platform() {
            PlatformName::Windows => "powershell.exe",
            _ => "sh",
        };

        let cwd = Path::new(base_path).join(project_name);
        let total_commands = template.len();

        for (idx, command) in template.iter().enumerate() {
            let current = idx as i8 + 1;
            Message::progress(command, current, total_commands as i8);

            Program::execute_command(program, command, &cwd, quite)
                .map_err(|e| anyhow!("Template command '{}' failed: {}", command, e))?;
        }

        Ok(())
    }
}
