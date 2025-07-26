use anyhow::{Result, anyhow, bail, ensure};

use crate::{
    cli::{TemplatesInfoArgs, TemplatesRemoveArgs},
    config::Config,
    platform::Platform,
    program::launch_program,
    templates::Templates,
    terminal::{Dialog, Message},
};

pub fn handle_new(templates: &mut Templates) -> Result<()> {
    let name = Dialog::ask_string("Name of new template?");
    if name.is_empty() {
        bail!("Incorrect name for a template.");
    }

    let mut commands: Vec<String> = Vec::new();
    loop {
        let command = Dialog::ask_string("Enter a command (press enter to finish):");
        if command.is_empty() {
            break;
        }
        commands.push(command.to_string());
    }

    ensure!(!commands.is_empty(), "No commands entered.");

    println!("Creating template...");
    templates.add_template(&name, commands)?;
    if templates.save().is_ok() {
        Message::done("Template created.");
    } else {
        bail!("Failed to save templates.");
    }
    Ok(())
}

pub fn handle_list(templates: &Templates) -> Result<()> {
    if templates.is_empty() {
        println!("No templates found.");
        return Ok(());
    }

    Message::title("Templates:");
    for template in templates.list_templates().iter() {
        Message::item(template);
    }
    Ok(())
}

pub fn handle_edit(config: &Config) -> Result<()> {
    let editor = &config.editor.program;
    if editor.is_empty() {
        bail!("Editor program name is not set in the configuration file.");
    }

    let path = Platform::get_templates_path();
    let mut editor_args = config.editor.args.clone();
    editor_args.push(path.to_str().unwrap().to_string());
    launch_program(editor, &editor_args, None, false, false).map_err(|e| anyhow!(e.to_string()))
}

pub fn handle_info(args: TemplatesInfoArgs, templates: &Templates) -> Result<()> {
    let name = args
        .name
        .ok_or_else(|| anyhow!("Provide a name of the template."))?;

    match templates.get_template(&name) {
        Some(template) => {
            Message::title("Commands of this template:");
            for command in template.iter() {
                Message::item(command);
            }
        }
        None => {
            bail!("Template not found.");
        }
    }
    Ok(())
}

pub fn handle_clear(templates: &mut Templates) -> Result<()> {
    if Dialog::ask("Clear all templates?", false) {
        templates.clear();
        templates.save()?;
        println!("All templates have been cleared.");
    } else {
        println!("Aborted.");
    }
    Ok(())
}

pub fn handle_remove(args: TemplatesRemoveArgs, templates: &mut Templates) -> Result<()> {
    let name = args
        .name
        .ok_or_else(|| anyhow!("Provide a name of template to delete."))?;
    templates.remove_template(&name).map_err(|e| anyhow!(e))?;
    templates.save().map_err(|e| anyhow!(e))?;
    Message::done("Removed.");
    Ok(())
}
