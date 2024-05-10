# Enjo changelog

## Next

- **WIP**: When you use the `list` command and have enabled the `hide_dots` option, Enjo will hide not only the directories that start with a dot symbol, but also system directories such as `.Trash-1000` or `System Volume Information`.
- **NEW**: You can clone your Git repositories through Enjo into your directory with projects via the `clone` command. Just use `enjo clone yourgit.com/user/repo` to clone a repository. See more with `enjo clone --help`.
- When Enjo generates a new configuration on Windows, instead of `code` it will use `code.cmd`.
- Some changes in messages.
- Some internal improvments.

## 0.1.1

- If you don't have a configuration file, Enjo will generate one and warn you about it.
- Default values for arguments are now hidden.
- Enjo will use a description from the package manifest rather than the one written in code.

## 0.1.0

First release of Enjo!