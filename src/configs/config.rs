use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Config {
    pub options: Options,
    pub programs: Programs
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Options {
    pub path: String,
    pub editor_args: Vec<String>,
    pub hide_dots: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { hide_dots: true, path: String::new(), editor_args: Vec::new() }
    }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Programs {
    pub editor: String,
    pub shell: String,
}