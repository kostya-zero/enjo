use crate::config::Config;
use crate::program::launch_program;
use crate::templates::Templates;
use crate::terminal::{Dialog, Message};
use anyhow::{Error, Result, anyhow};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub enum CompletionResult {
    Found,
    FoundSimilar(String),
    Nothing,
}

pub fn autocomplete(word: &str, words_list: Vec<&str>) -> Option<String> {
    let suggested = suggest_completion(word, words_list.clone());

    match suggested {
        CompletionResult::Found => Some(word.to_string()),
        CompletionResult::FoundSimilar(name) => {
            let answer = Dialog::ask(&format!("Did you mean '{name}'?"), true);
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

pub fn apply_template(
    template_name: &str,
    config: &Config,
    project_name: &str,
    quiet: bool,
) -> Result<(), Error> {
    let templates = Templates::load().map_err(|e| anyhow!("Failed to load templates: {}", e))?;
    let template = templates
        .get_template(template_name)
        .ok_or_else(|| anyhow!("Template '{}' not found", template_name))?;

    Message::print("Generating project from template...");

    let program = &config.shell.program;

    let cwd = Path::new(&config.options.projects_directory).join(project_name);
    let total_commands = template.len();

    for (idx, command) in template.iter().enumerate() {
        let current = idx as i8 + 1;
        Message::progress(command, current, total_commands as i8);

        let mut args = config.shell.args.clone();
        args.push(command.clone());

        launch_program(program, &args, Some(cwd.to_str().unwrap()), false, quiet)
            .map_err(|e| anyhow!("Template command '{}' failed: {}", command, e))?;
    }

    Ok(())
}
