# Enjo
Enjo is a minimalist workspace assistant tool that allows you to quickly manager your projects.

### Installation

Download binary from releases on GitHub and place it in place that exists in PATH.

### Usage

To see full list of available arguments, run Enjo with argument `help`:

```shell
enjo help
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
You can open editor with configuration file opened by running Enjo with `config` argument:

```shell
enjo config
```

