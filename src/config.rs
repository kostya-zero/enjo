use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    path: Option<String>,
    editor: Option<String>,
    editor_args: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: Some(String::new()),
            editor: Some(String::from("nvim")),
            editor_args: Some(Vec::new()),
        }
    }
}

impl Config {
    pub fn get_path(&self) -> String {
        match self.path.clone() {
            Some(i) => i,
            None => String::new(),
        }
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = Some(String::from(path));
    }

    pub fn get_editor(&self) -> String {
        match self.editor.clone() {
            Some(i) => i,
            None => String::new(),
        }
    }

    pub fn set_editor(&mut self, editor: &str) {
        self.editor = Some(String::from(editor));
    }

    pub fn get_editor_args(&self) -> Vec<&str> {
        match &self.editor_args {
            Some(vect) => vect.iter().map(|i| i.as_str()).collect(),
            None => Vec::new(),
        }
    }


}
