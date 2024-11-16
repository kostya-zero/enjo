# Enjo ![Crates.io Version](https://img.shields.io/crates/v/enjo) ![GitHub branch check runs](https://img.shields.io/github/check-runs/kostya-zero/enjo/main)

Enjo is a powerful CLI application designed to streamline project management directly from your terminal.

## Requirements

- **OS**: Windows, Linux, macOS __(only x86_64 available in precompiled binaries)__
- **Nerd Font** (for icons)

## Installation

Download the latest version of Enjo from [Releases](https://github.com/kostya-zero/enjo/releases).

If you wish to install Enjo via `cargo`, ensure you have the Rust toolchain. Then, use `cargo` to install:

```shell
# Compile and install Enjo.
cargo install enjo

# For precompiled binaries (if you have cargo-binstall installed).
cargo binstall enjo
```
If you want to build Enjo, please visit [Building Enjo](docs/BUILDING.md).

## Getting Started

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
