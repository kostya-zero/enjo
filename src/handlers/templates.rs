use anyhow::{Result, anyhow, bail, ensure};

use crate::{
    cli::{TemplatesInfoArgs, TemplatesRemoveArgs},
    config::Config,
    platform::Platform,
    program::launch_program,
    templates::Templates,
    terminal::{ask_dialog, ask_string_dialog, print_done, print_item, print_title},
};

pub fn handle_new(templates: &mut Templates) -> Result<()> {
    let name = ask_string_dialog("Name of new template?");
    if name.is_empty() {
        bail!("Incorrect name for a template.");
    }

    let mut commands: Vec<String> = Vec::new();
    loop {
        let command = ask_string_dialog("Enter a command (press enter to finish):");
        if command.is_empty() {
            break;
        }
        commands.push(command.to_string());
    }

    ensure!(!commands.is_empty(), "No commands entered.");

    println!("Creating template...");
    templates.add_template(&name, commands)?;
    if templates.save().is_ok() {
        print_done("Created.");
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

    print_title("Templates:");
    for template in templates.list_templates().iter() {
        print_item(template);
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
            print_title("Commands of this template:");
            for command in template.iter() {
                print_item(command);
            }
        }
        None => {
            bail!("Template not found.");
        }
    }
    Ok(())
}

pub fn handle_clear(templates: &mut Templates) -> Result<()> {
    if ask_dialog("Clear all templates?", false) {
        templates.clear();
        templates.save()?;
        print_done("Cleared.");
    } else {
        print_done("Aborted.");
    }
    Ok(())
}

pub fn handle_remove(args: TemplatesRemoveArgs, templates: &mut Templates) -> Result<()> {
    let name = args
        .name
        .ok_or_else(|| anyhow!("Provide a name of template to delete."))?;
    templates.remove_template(&name).map_err(|e| anyhow!(e))?;
    templates.save().map_err(|e| anyhow!(e))?;
    print_done("Removed.");
    Ok(())
}
