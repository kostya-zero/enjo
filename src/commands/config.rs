use anyhow::{Result, anyhow, bail};

use crate::{
    config::Config,
    platform::Platform,
    program::{LaunchOptions, launch_program},
    terminal::{ask_dialog, print_done},
};

pub fn handle_path() -> Result<()> {
    println!("{}", Platform::get_config_path().to_str().unwrap());
    Ok(())
}

pub fn handle_edit() -> Result<()> {
    let config = Config::load(Platform::get_config_path())?;
    let editor = &config.editor.program;
    if editor.is_empty() {
        bail!("Editor program name is not set in the configuration file.");
    }

    let path = Platform::get_config_path();
    let mut editor_args = config.editor.args.clone();
    editor_args.push(path.to_str().unwrap().to_string());

    let launch_options = LaunchOptions {
        program: editor.to_string(),
        args: editor_args,
        fork_mode: false,
        quiet: false,
        cwd: None,
        env: None,
    };

    launch_program(launch_options).map_err(|e| anyhow!(e.to_string()))
}

pub fn handle_reset() -> Result<()> {
    let mut config = Config::load(Platform::get_config_path())?;
    if ask_dialog("Reset your current configuration?", false) {
        config.reset();
        config.save(Platform::get_config_path())?;
        print_done("Reset.");
    } else {
        print_done("Aborted.");
    }
    Ok(())
}
