use std::default;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    options: Options,
    programs: Programs
}

#[derive(Deserialize, Serialize, Default)]
pub struct Options {
    path: Option<String>,
    editor_args: Option<Vec<String>>,
    hide_dots: Option<bool>
}

#[derive(Deserialize, Serialize, Default)]
pub struct Programs {
    editor: Option<String>,
    shell: Option<String>,
}

impl Config {
    pub fn get_path(&self) -> String {
        self.options.path.clone().unwrap_or_default()
    }

    pub fn set_path(&mut self, path: &str) {
        self.options.path = Some(String::from(path));
    }

    pub fn get_editor(&self) -> String {
        self.programs.editor.clone().unwrap_or_default()
    }

    pub fn set_editor(&mut self, editor: &str) {
        self.programs.editor = Some(String::from(editor));
    }

    pub fn get_shell(&self) -> String {
        self.programs.shell.clone().unwrap_or_default()
    }

    pub fn set_shell(&mut self, shell: &str) {
        self.programs.shell = Some(String::from(shell));
    }

    pub fn get_hide_dots(&self) -> bool {
        self.options.hide_dots.unwrap_or_default()
    }

    pub fn set_hide_dots(&mut self, state: bool) {
        self.options.hide_dots = Some(state);
    }

    pub fn get_editor_args(&self) -> Vec<&str> {
        self.options.editor_args
            .as_ref()
            .map(|vect| vect.iter().map(|i| i.as_str()).collect())
            .unwrap_or_default()
    }
}
