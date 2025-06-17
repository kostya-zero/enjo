use crate::config::Config;
use crate::library::Library;
use crate::platform::Platform;
use crate::program::launch_program;
use crate::templates::Templates;
use crate::terminal::{Dialog, Message};
use crate::utils;
use anyhow::{Error, Result, anyhow, bail};
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

pub fn apply_template(
    template_name: &str,
    config: &Config,
    project_name: &str,
    quite: bool,
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

        launch_program(program, &args, Some(cwd.to_str().unwrap()), quite, false)
            .map_err(|e| anyhow!("Template command '{}' failed: {}", command, e))?;
    }

    Ok(())
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

pub fn resolve_project_name(
    project_name: &str,
    config: &Config,
    projects: &Library,
) -> Result<String> {
    if project_name == "-" {
        if config.recent.recent_project.is_empty() {
            bail!("No project was opened recently.")
        }
        Ok(config.recent.recent_project.clone())
    } else if config.autocomplete.enabled {
        let name = utils::autocomplete(project_name, projects.get_names());
        if let Some(name) = name {
            Ok(name)
        } else {
            bail!("Project not found.")
        }
    } else {
        Ok(project_name.to_string())
    }
}
