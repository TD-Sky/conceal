<h1 align="center">🌠 conceal 🌃</h1>

<p align="center">
  <a href="https://crates.io/crates/conceal"><img src="https://img.shields.io/crates/v/conceal.svg?style=flat-square" /></a>
  <a href="https://crates.io/crates/conceal"><img src="https://img.shields.io/crates/d/conceal.svg?style=flat-square" /></a>
</p>

## Introduction

**conceal** is a command line tool based on [trash-rs](https://github.com/Byron/trash-rs) which implements [The FreeDesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/trashspec-latest.html).

You can use **conceal** to:
- **Throw** the files **in** the trash bin;
- **List** all the discarded files;
- **Delete** all the discarded files permanently;
- **Restore** files discarded under the current directory.



## Finder

`conceal restore` use [skim](https://github.com/lotabout/skim) and [**fzf**](https://github.com/junegunn/fzf) (default) as finder.
You can choose either one with the `--finder` option whose value is also controlled by environment variable `CONCEAL_FINDER`.

> Value priority: `--finder` > `CONCEAL_FINDER`

Refer to `conceal restore -h` for more information.



## Installation

### Cargo

You need to install finder [skim](https://github.com/lotabout/skim#installation) or [fzf](https://github.com/junegunn/fzf#installation) before using **conceal**.

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

[![demo](https://asciinema.org/a/629369.svg)](https://asciinema.org/a/629369?speed=2)

**`Tab` is the multi-selection and multi-cancel key.**
