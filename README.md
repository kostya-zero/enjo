# Enjo ![Crates.io Version](https://img.shields.io/crates/v/enjo) ![GitHub branch check runs](https://img.shields.io/github/check-runs/kostya-zero/enjo/main)

Enjo is a command-line tool designed for managing your projects.
It offers a simple and user-friendly interface for creating, opening, and deleting projects.
Enjo is available for Windows, Linux, and macOS (compatibility with *BSD systems is not guaranteed).

## Installation

We recommend using [Cargo](https://doc.rust-lang.org/cargo/) to install Enjo. You can install Enjo with the following commands:

```shell
# Compile and install Enjo.
cargo install enjo

# Install precompiled binaries (requires cargo-binstall).
cargo binstall enjo
```

You can also install Enjo from [GitHub Releases](https://github.com/kostya-zero/enjo/releases). If you prefer to build Enjo from source, please refer to the [Building Enjo](docs/BUILDING.md) guide.

## Usage

Before using Enjo, configure it according to your workspace setup. Detailed configuration options are available in the [Configuration Manual](docs/CONFIGURATION.md).

### List Projects

Enjo simplifies project management. To view a list of your projects, use the `list` subcommand:

```shell
enjo list
```

> [!NOTE]
> By default, Enjo hides projects whose names start with a dot (e.g., `.hidden_project`). You can change this behavior by configuring the `display_hidden` parameter as described in the [Configuration Manual](docs/CONFIGURATION.md).

### Managing Projects

Creating and deleting projects with Enjo is straightforward:

```shell
# Create a new project.
enjo new bookshelf

# Delete an existing project.
enjo delete bookshelf
```

### Working with Projects

Open project directories directly in your configured editor or shell using the `open` subcommand:

```shell
# Open the project in your editor.
enjo open bookshelf

# Open the project in your shell.
enjo open bookshelf --shell
```

### Quick Help

For assistance with commands, use the `--help` flag:

```shell
# General help.
enjo --help

# Help for a specific subcommand.
enjo config --help
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
