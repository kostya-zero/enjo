use dialoguer::{
    Confirm, Input,
    console::{Style, style},
    theme::{ColorfulTheme, Theme},
};

use crate::colors;

pub struct Message;

impl Message {
    fn print_message(prefix: &str, color_code: &str, msg: &str) {
        println!("{}{}{}\x1b[0m: {}", color_code, colors::BOLD, prefix, msg);
    }

    pub fn error(msg: &str) {
        eprintln!(
            "{}{}error{}: {}",
            colors::RED,
            colors::BOLD,
            colors::RESET,
            msg
        )
    }

    pub fn done(msg: &str) {
        Self::print_message("done", colors::GREEN, msg);
    }

    pub fn busy(msg: &str) {
        Self::print_message("busy", colors::WHITE, msg);
    }

    pub fn info(msg: &str) {
        Self::print_message("info", colors::WHITE, msg);
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
