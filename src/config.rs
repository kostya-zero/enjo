use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    path: Option<String>,
    editor: Option<String>,
    shell: Option<String>,
    editor_args: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: Some(String::new()),
            editor: Some(String::from("nvim")),
            shell: Some(String::from("bash")),
            editor_args: Some(Vec::new()),
        }
    }
}

impl Config {
    pub fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = Some(String::from(path));
    }

    pub fn get_editor(&self) -> Option<String> {
        self.editor.clone()
    }

    pub fn set_editor(&mut self, editor: &str) {
        self.editor = Some(String::from(editor));
    }

    pub fn get_shell(&self) -> Option<String> {
        self.shell.clone()
    }

    pub fn set_shell(&mut self, shell: &str) {
        self.shell = Some(String::from(shell));
    }

    pub fn get_editor_args(&self) -> Option<Vec<&str>> {
        self.editor_args
            .as_ref()
            .map(|vect| vect.iter().map(|i| i.as_str()).collect())
    }
}
