# Configuration manual

With this manual you can easily configure Enjo for your workspace.

### Configuration file

If you don't have configuration file in your file system, Enjo will generate it in this locations:

- **Windows**: `%USERPROFILE%\.enjo\config.toml`
- **Linux** and **macOS**: `$HOME/.enjo/config.toml`

Enjo will generate configuration based on your environment settings like `EDITOR` and `SHELL` environment variables.

> ⚠️ If you are using *VS Code* or *VS Codium* on Windows, Enjo will set `editor` field to `code.cmd` and `codium.cmd` in configuration, because for some reason Enjo cant find `code` and `codium`. Also, Enjo will add a single dot to `editor_args` to tell *VS Code* and *VS Codium* to open this directory.

### Default configuration structure

```toml
[options]
path = '/home/user'
editor_args = []
display_hidden = false

[programs]
editor = "nvim"
shell = "bash"
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
- `editor_args` - Arguments that will be passed to the editor. By default this field is empty, but for users of *VS Code* and *VS Codium* it will contain a single dot.
- `display_hidden` - This options determines the display of hidden directories. By default set to `false`.

### `programs`

- `editor` - Name of executable that will be used as editor to open projects and configuration file. You set it like name of executable (e.g. `nvim`) or as full path to executable (e.g. `/usr/bin/nvim`).
- `shell` - Name of executable that will be used as shell to open projects. You set it like name of executable (e.g. `bash`) or as full path to executable (e.g. `/usr/bin/bash`).

