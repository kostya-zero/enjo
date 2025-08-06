# Configuration Manual

This manual provides guidance on configuring Enjo for your workspace.

### Configuration File

If a configuration file does not exist in your file system, Enjo will generate one in the following locations:

- **Windows**: `%USERPROFILE%\enjo\config.toml`
- **Linux** and **macOS**: `$HOME/.config/enjo/config.toml`

Enjo generates the configuration based on your environment settings, such as the `EDITOR` and `SHELL` environment variables.

>[!NOTE]
> On Windows, Enjo appends `.cmd` to the `program` field for specific editors. This is because Enjo requires the `.cmd` files to launch these editors. The affected editors are:
>
> - Visual Studio Code
> - Visual Studio Code - Insiders
> - Code - OSS
> - VS Codium
> - Windsurf

### Default configuration structure

```toml
[options]
projects_directory = '/home/user'
display_hidden = false

[editor]
program = "nvim"
fork_mode = false
args = []

[shell]
program = "bash"
args = ["-c"]

[recent]
enabled = true
recent_project = "example"

[autocomplete]
enabled = true
always_accept = true
```

For more information about the fields in the configuration, refer to the [Parameters section](#parameters).

### Manage configuration

Enjo allows you to manage the configuration through the `config` subcommand. Here is a list of available actions:

- `edit` - Opens the configuration file in the editor specified in the `[editor]` section of the configuration. This allows you to manually edit the configuration settings.
- `path` - Gets the path to the configuration file.
- `reset` - Resets the configuration to its default settings.

# Parameters

This section details the configuration parameters available in the `config.toml` file, organized by their respective sections.

### `options`

- `projects_directory` - Path to the directory containing your projects. By default, it uses the path to the user's home directory.
- `display_hidden` - Controls whether hidden directories are displayed. By default, set to `false`.

### `editor`

- `program` - The name of the executable to be used as an editor to open projects and the configuration file. You can specify it as the name of an executable (e.g., `nvim`) or as an absolute path to the executable (e.g., `/usr/bin/nvim`).
- `fork_mode` - Determines whether the editor should run as a separate process. Set this to `true` if you want Enjo to launch the editor and immediately return control to the terminal (useful for GUI editors).
- `args` - Arguments to be passed to the editor. By default, this field is empty, but for _VS Code_, _VS Codium_, _Windsurf_, and _Zed_, it will contain a dot (`.`) to open the current directory.

### `shell`

- `program` - Name of the executable that will be used as a shell to open projects. You can set it as the name of an executable (e.g., `bash`) or as an absolute path to the executable (e.g., `/usr/bin/bash`).
- `args` - Arguments to be passed to run commands using the shell. By default, this field is set to value determined by user's and `EDITOR` variable.

### `recent`

- `enabled` - Controls whether the recent projects feature is enabled. By default, set to `true`.
- `recent_project` - Name of the most recent project. This field is used to store the name of the most recently opened project.

### `autocomplete`

- `enabled` - Controls whether the autocomplete feature is enabled. By default, set to `true`.
- `always_accept` - Determines whether the autocomplete feature should automatically accept the suggestion. If set to `true`, it will automatically select the suggestion. By default, this is set to `true`.
