# Enjo ![Crates.io Version](https://img.shields.io/crates/v/enjo) ![GitHub branch check runs](https://img.shields.io/github/check-runs/kostya-zero/enjo/main)

Enjo is a command-line tool for managing your projects. It provides a simple and user-friendly interface for creating, opening, and deleting projects.

## Requirements

- **OS**: Windows, Linux, macOS (compaitability with *BSD systems is not guaranteed).
- **Nerd Font** (for icons)

## Installation

We recommend to use [Cargo](https://doc.rust-lang.org/cargo/) to install Enjo. You can install Enjo using the following commands:

```shell
# Compile and install Enjo.
cargo install enjo

# For precompiled binaries (if you have cargo-binstall installed).
cargo binstall enjo
```

Also you can install Enjo from [GitHub Releases](https://github.com/kostya-zero/enjo/releases). If you want to build Enjo, please visit [Building Enjo](docs/BUILDING.md).

## Usage

Before using Enjo, configure it based on your workspace setup. Full configuration options are available in the [Configuration Manual](docs/CONFIGURATION.md).

### List Projects

Enjo allows easy management of your projects. To view a list of projects, use the `list` subcommand:

```shell
enjo list
```

> [!NOTE]
> By default, Enjo will hide projects that begin with a dot (e.g., .`hidden_project`). You can adjust this setting by configuring the `hide_dots` parameter as outlined in the [Configuration Manual](docs/CONFIGURATION.md).

### Managing Projects

Creating and deleting projects in Enjo is straightforward:

```shell
# Create a new project.
enjo new bookshelf

# Delete an existing project.
enjo delete bookshelf
```

### Working with Projects

Open project directories directly in your editor or shell using the `open` subcommand:

```shell
# Open the project in your editor.
enjo open bookshelf

# Open the project in your shell.
enjo open bookshelf --shell
```

### Quick Help

For help with commands, use the `--help` flag:

```shell
# General help.
enjo --help

# Help for a specific subcommand.
enjo config --help
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
