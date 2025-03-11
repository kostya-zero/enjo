use crate::platform::{Platform, PlatformName};
use crate::program::Program;
use crate::terminal::{Dialog, Message};
use crate::{config::Config, storage::Storage};
use anyhow::{Error, Result, anyhow, bail};
use std::path::Path;
use std::process::{Command, Stdio};
use crate::colors::{BOLD, RESET, WHITE};

#[derive(Debug, Eq, PartialEq)]
pub enum CompletionResult {
    Found,
    FoundSimilar(String),
    Nothing,
}

pub struct Utils;
impl Utils {
    pub fn launch_program(
        program: &str,
        args: Vec<String>,
        cwd: &str,
        fork_mode: bool,
    ) -> Result<()> {
        let mut proc = Program::new(program);
        if !cwd.is_empty() {
            proc.set_cwd(cwd);
        }
        proc.set_fork_mode(fork_mode);
        proc.set_args(args);
        if let Err(e) = proc.run() {
            Err(anyhow!(e.to_string()))
        } else {
            Ok(())
        }
    }

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
            let templates = Storage::new();
            if templates.save_storage().is_err() {
                bail!("Failed to generate storage file.");
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
        let templates = Storage::load_storage()?;
        let template = templates
            .get_template(template_name)
            .map_err(|_| anyhow!("Template not found"))?;

        Message::info("Generating project from template...");

        let program = match Platform::get_platform() {
            PlatformName::Windows => "powershell.exe",
            _ => "sh",
        };

        let cwd = Path::new(base_path).join(project_name);
        let total_commands = template.len();

        for (idx, command) in template.iter().enumerate() {
            println!("{}{}[{}/{}]{} {}", WHITE, BOLD, idx + 1, total_commands, RESET, command);

            Self::execute_template_command(program, command, &cwd, quite)
                .map_err(|e| anyhow!("Template command '{}' failed: {}", command, e))?;
        }

        Ok(())
    }

    fn execute_template_command(
        program: &str,
        command: &str,
        cwd: &Path,
        quite: bool,
    ) -> Result<(), Error> {
        let mut cmd = Command::new(program);
        cmd.args(["-c", command]).current_dir(cwd);

        if !quite {
            cmd.stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        let output = cmd.output()?;

        if !output.status.success() {
            bail!("Command failed with exit code: {}", output.status);
        }

        Ok(())
    }
}
