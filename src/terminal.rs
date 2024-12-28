use std::process::exit;

use dialoguer::{
    console::style,
    theme::{ColorfulTheme, Theme},
    Confirm, Input,
};

pub struct Message;

impl Message {
    fn print_message(prefix: &str, color_code: &str, msg: &str) {
        println!("\x1b[{}m{}\x1b[0m {}", color_code, prefix, msg);
    }

    pub fn error(msg: &str) {
        eprintln!("\x1b[91m\x1b[0m {}", msg)
    }

    pub fn done(msg: &str) {
        Self::print_message("󰸞", "92", msg);
    }

    pub fn busy(msg: &str) {
        Self::print_message("󰦖", "97", msg);
    }

    pub fn running(msg: &str) {
        Self::print_message("", "97", msg);
    }

    pub fn info(msg: &str) {
        Self::print_message("", "97", msg);
    }

    pub fn list_title(msg: &str) {
        Self::print_message("", "97", msg);
    }

    pub fn item(msg: &str) {
        println!(" {}", msg);
    }

    pub fn fail(msg: &str) {
        Self::error(msg);
        exit(1);
    }
}

pub struct Dialog;

impl Dialog {
    fn get_theme() -> impl Theme {
        ColorfulTheme {
            prompt_prefix: style("?".to_string()).for_stdout().cyan(),
            success_prefix: style("󰸞".to_string()).for_stdout().green(),
            error_prefix: style("".to_string()).for_stderr().red(),
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
