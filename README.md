# Enjo
Enjo is a minimalist workspace assistant tool that allows you to quickly manage your projects.

### Installation

You have 3 ways to install Enjo.

##### Binary from releases

You can download latest releases on GitHub and place binary in directory that was added to `PATH`.

##### 

### Usage

Before you start, You should configure your Enjo settings. Go to [configuration](#configuration) section for help.

Enjo allows you to manage your projects and work with it.
You can get list of projects by using `list` subcommand.

```shell
enjo list
```

Also you can create and delete projects.

```shell
# Use `new` to create new project.
enjo new bookshelf

# Use `delete` to delete project.
enjo delete bookshelf
```

Enjo allows you to quickly jump into your project with editor that specified in your configuration. 
Use `open` subcommand and then specify name of project.

```
enjo open bookshelf
```

If you want to get help about something, use `--help` argument.

```shell
enjo --help

# It's also works with subcommands
enjo config --help
```

### Configuration

When you start Enjo for the first time, it will generate default configurations with custom options based on your environment.
For example, if you have `EDITOR` variable set in your environment, Enjo will use it value as value for `editor` option in configuration file.

Default configuration structure:

```toml
path = "/home/user"
editor = "nvim"
editor_args = []
```

Configuration file is located at root of your user home directory and named as `.enjo.toml`.
You can open editor with configuration file opened by running Enjo with `config edit` argument:

```shell
enjo config edit
```

To reset your config use `config reset` subcommand with provided `--yes` argument.


```shell
enjo config reset
```

> `--yes` required because it shows your agreement to reset configuration.

