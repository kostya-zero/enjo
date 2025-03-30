use std::time::Duration;

use dialoguer::{
    Confirm, Input,
    console::{Style, style},
    theme::{ColorfulTheme, Theme},
};
use indicatif::{ProgressBar, ProgressStyle};

use crate::colors;

pub struct Message;

impl Message {
    pub fn error(msg: &str) {
        eprintln!(
            "{}{}error{}: {}",
            colors::RED,
            colors::BOLD,
            colors::RESET,
            msg
        )
    }

    pub fn print(msg: &str) {
        println!("{}", msg);
    }

    pub fn progress(msg: &str, current: i8, total: i8) {
        println!("{}{}[{}/{}]{} {}", colors::WHITE, colors::BOLD, current, total, colors::RESET, msg);
    }

    pub fn title(msg: &str) {
        println!("{}{}{}", colors::BOLD, msg, colors::RESET);
    }

    pub fn item(msg: &str) {
        println!(" {}", msg);
    }
}

pub struct Dialog;

impl Dialog {
    fn get_theme() -> impl Theme {
        ColorfulTheme {
            prompt_prefix: style("?".to_string()).for_stdout().yellow(),
            success_prefix: style("✔".to_string()).for_stdout().green(),
            error_prefix: style("✘".to_string()).for_stderr().red(),
            defaults_style: Style::new().for_stdout().dim().white(),
            ..Default::default()
        }
    }

    pub fn ask(question: &str, default: bool) -> bool {
        Confirm::with_theme(&Self::get_theme())
            .with_prompt(question)
            .default(default)
            .show_default(true)
            .report(false)
            .interact()
            .unwrap()
    }

    pub fn ask_string(question: &str) -> String {
        Input::<String>::with_theme(&Self::get_theme())
            .with_prompt(question)
            .default(String::new())
            .report(false)
            .interact_text()
            .unwrap()
    }
}

pub fn create_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"])
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(150));
    spinner
}
