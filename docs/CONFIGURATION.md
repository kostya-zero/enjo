# Configuration manual

With this manual you can easily configure Enjo for your workspace.

### Configuration file

If you don't have configuration file in your file system, Enjo will generate it in this locations:

- **Windows**: `%USERPROFILE%\.enjo\config.toml`
- **Linux** and **macOS**: `$HOME/.enjo/config.toml`

Enjo will generate configuration based on your environment settings like `EDITOR` and `SHELL` environment variables.

> [!WARNING]
> If you are using _VS Code_ or _VS Codium_ on Windows, Enjo will set `editor` field to `code.cmd` and `codium.cmd` in configuration, because for some reason Enjo cant find `code` and `codium`. Also, Enjo will add a single dot to `editor_args` to tell _VS Code_, _VS Codium_ and _Zed_ to open this directory.

### Default configuration structure

```toml
[options]
path = '/home/user'
display_hidden = false
autocomplete = false

[editor]
program = "nvim"
fork_mode = false
args = []

[shell]
program = "fish"
```

For more information about fields in configuration goto [parameters section](#parameters).

### Manage configuration

Enjo allows to manage configuration through `config` subcommand. This is a list of available actions:

- `edit` - Open editor to configure configuration file.
- `path` - Get path to the configuration file.
- `reset` - Reset configuration.

# Parameters

### `options`

- `path` - Path to directory with your projects. But default it uses path to user's home directory.
- `autocomplete` - Enables autocomplete functionality. Disabled by default.
- `display_hidden` - This options determines the display of hidden directories. By default, set to `false`.

### `editor`

- `program` - The name of the executable to be used as an editor to open projects and configuration file. You can specify it as the name of an executable (e.g., `nvim`) or as an absolute path to the executable (e.g., `/usr/bin/nvim`).
- `fork_mode` - Determines whether the editor should run as a separate process.
- `args` - Arguments to be passed to the editor. By default this field is empty, but for _VS Code_ and _VS Codium_ it will contain a dot.

### `shell`

- `program` - Name of executable that will be used as shell to open projects. You set it like name of executable (e.g. `bash`) or as absolute path to executable (e.g. `/usr/bin/bash`).
