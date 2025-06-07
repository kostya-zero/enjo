use colored::Colorize;
use dialoguer::{
    console::{style, Style}, theme::{ColorfulTheme, Theme},
    Confirm,
    Input,
};

pub struct Message;

impl Message {
    pub fn error(msg: &str) {
        eprintln!(
            "{}: {}",
            "err".red(),
            msg
        )
    }

    pub fn print(msg: &str) {
        println!("{}", msg);
    }

    pub fn progress(msg: &str, current: i8, total: i8) {
        println!(
            "{} {}",
            format!("[{}/{}]", current, total).white().bold(),
            msg
        );
    }

    pub fn title(msg: &str) {
        println!("{}", msg.white().bold());
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
