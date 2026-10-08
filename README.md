# cheat

Display custom cheatsheets.

## Usage

Will look for cheatsheets at the path specified in the environment variable `CHEAT_PATH`, or by
default, in `~/.config/cheat/`. See [Cheatsheet format](#cheatsheet-format) for more info.

### List cheatsheets:

`cheat [-p <path>]`

### Display cheatsheet/section:

`cheat [-p <path>] <section-path>`

### Help:

`cheat -h`

### Arguments

`-p <path>`: Look for cheatsheets in `path` instead of the default path.

`<section-path>`: A path of the format `<sheet>[.<section>[.<subsection>[...]]]`, where `sheet` is
the name of the cheatsheet file without the TOML file extension, and `section`, `subsection` etc.
are keys to the TOML table in the sheet file.

## Cheatsheet format

Cheatsheets are stored in TOML format. Non-TOML files will be ignored.

```toml
# vim.toml
[commands]
":sav" = "save file as"
":ter" = "open terminal window"
# ...

[commands.buffer]
":bn" = "next buffer"
":bp" = "previous buffer"
# ...
```
