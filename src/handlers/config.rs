use anyhow::{Result, anyhow, bail};

use crate::{config::Config, platform::Platform, program::launch_program, terminal::Dialog};

pub fn handle_path() -> Result<()> {
    println!("{}", Platform::get_config_path().to_str().unwrap());
    Ok(())
}

pub fn handle_edit(config: &Config) -> Result<()> {
    let editor = &config.editor.program;
    if editor.is_empty() {
        bail!("Editor program name is not set in the configuration file.");
    }

    let path = Platform::get_config_path();
    let mut editor_args = config.editor.args.clone();
    editor_args.push(path.to_str().unwrap().to_string());
    launch_program(editor, &editor_args, None, false, false).map_err(|e| anyhow!(e.to_string()))
}

pub fn handle_reset(config: &mut Config) -> Result<()> {
    if Dialog::ask("Reset your current configuration?", false) {
        config.reset();
        config.save()?;
        println!("The configuration has been reset.");
    } else {
        println!("Aborted.");
    }
    Ok(())
}
