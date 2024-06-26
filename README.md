# Enjo

Enjo is a minimalist workspace management tool that allows you to quickly manage your projects.
It has functionality to manage and open projects in editor/shell.
Enjo will be useful for those who use the terminal frequently and want to quickly jump to their projects.

Main goal of this project is to provide fast and useful assistant for developers. We will be glad to hear your suggestions.

## Installation

If you have installed Rust toolchain, you can install Enjo via `cargo`:

```shell
cargo install enjo --locked
```

If not, you can install Enjo by downloading archive for your system from releases.
If you want to build Enjo, please visit [Building Enjo](docs/BUILDING.md).

> ⚠️ Before using Enjo, you need to configure it based on your workspace. All options are described in [configuration manual](docs/CONFIGURATION.md).

## Usage

Enjo allows you to manage your projects and work with it.
You can get list of projects by using `list` subcommand.

```shell
enjo list
```

> ⚠️ By default Enjo will not display projects with name starting with dots. Please confgiure `hide_dots` parameter according to [configuration manual](docs/CONFIGURATION.md).

You can create and delete your projects through Enjo.

```shell
# Use `new` to create new project.
enjo new bookshelf

# Use `delete` to delete project.
enjo delete bookshelf
```

With Enjo you can open project directory with editor or shell.
Use `open` subcommand and then specify name of project. If you need to open shell, add `--shell` argument

```shell
# Open project in editor.
enjo open bookshelf

# Open project in shell.
enjo open bookshelf --shell
```

If you want to get help about something, use `--help` argument.

```shell
# Show regular help
enjo --help

# It's also works with subcommands
enjo config --help
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
