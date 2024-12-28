# Enjo changelog

## Next

TBD

## 0.5.2

- Fixed bug when trying to create a new project without specified template Enjo was showing an `Template not found` message.
- Some changes in output icons.

## 0.5.1

- Remove spinner animation when removing project.
- Project directory will be removed if template is not found.
- Enjo will return an error if template command exited with non-zero code.
- Performance improvements.

## 0.5.0

- **Autocomplete**. Enjo will now complete project names in the `open` and `delete` commands. This option can be configured in the configuration file.
- The templates file have been moved to a storage file. You need to add all of your templates again.
- You can open your recent project using dash symbol (`-`) in the `open` command. Example: `enjo open -` will open your recent project.
- Added support for Windsurf editor when generating default configuration.

## 0.4.0

- **Templates**. Now you can create templates to generate projects from. Use `enjo templates --help` for help.
- New global argument `--hidden` allows to display hidden even if `display_option` is set to `false`.
- Fixed `unknown t switch` message from Git.
- Increased stability of `clone` command.
- Enjo will warn you if you are cloning repository which name starts with dot.

## 0.3.0

- **The configuration file has been restructured.** If you used Enjo before version `0.2.1`, you need to reset your configuration file. See [configuration manual](docs/CONFIGURATION.md) for more details.
- Added `fork_mode` option to the `editor` section. This option determines whether the editor should be started as a separate process or whether Enjo needs to wait until it exits.
- Added new subcommand `rename` that allows you to rename project.

## 0.2.1

- System directories will be hidden regardless of the value of the `display_hidden` parameter.
- Confirmation to reset the configuration is now implemented through a dialog instead of an argument.
- When deleting a project, Enjo will ask for confirmation if the project is not empty.
- If `zed`, `code` or `codium` is used as the editor, Enjo will add a dot to the editor arguments.
- Slight changes to the wording of the messages.

## 0.2.0

- You can clone your projects from remote Git repository with `clone` command.
- You can hide hidden and system directories from list of projects with `display_hidden` option in your configuration file.
- When Enjo generates a new configuration on Windows, it will use `code.cmd` instead of `code`.
- Some changes in messages.
- Some internal improvements.

## 0.1.1

- If you don't have a configuration file, Enjo will generate one and warn you about it.
- Default values for arguments are now hidden.
- Enjo will use a description from the package manifest rather than the one written in code.

## 0.1.0

First release of Enjo!
