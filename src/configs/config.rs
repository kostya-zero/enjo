use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Config {
    pub options: Options,
    pub programs: Programs
}

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Options {
    pub path: String,
    pub editor_args: Vec<String>,
    pub hide_dots: bool,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Programs {
    pub editor: String,
    pub shell: String,
}