<h1 align="center">🌠 conceal 🌃</h1>



## Introduction

**conceal** is a command line tool based on [trash-rs](https://github.com/Byron/trash-rs) which implements [The FreeDesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/trashspec-latest.html).

You can use **conceal** to:
- **Throw** the files **in** the trash bin;
- **List** all the discarded files;
- **Delete** all the discarded files permanently;
- **Restore** files discarded under the current directory.



## Installation

### Cargo

**conceal** supports [skim](https://github.com/lotabout/skim) and [fzf](https://github.com/junegunn/fzf)
as finder. You can choose either one with the environment variable
`CONCEAL_FINDER` (value: `skim` | `fzf`). The default one is `skim`.

Also, you can override the `finder` argument when calling command.
Refer to `conceal restore -h` for more information.

You need to install finder before using **conceal**.

```bash
$ cargo install --git 'https://github.com/TD-Sky/conceal'
```

### AUR

```bash
$ paru -S conceal
```

or binary package:

```bash
$ paru -S conceal-bin
```



## Usage

There are two binaries: `cnc` and `conceal`.

`cnc` is to put files into recycle bin; `conceal` is to operate the recycle bin.

[![demo](https://asciinema.org/a/571859.svg)](https://asciinema.org/a/571859?speed=2)

**`Tab` is the multi-selection and multi-cancel key !!**
