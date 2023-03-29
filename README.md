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

**First of all**, install [skim](https://github.com/lotabout/skim#installation).

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



## License

The MIT License ([MIT](https://opensource.org/licenses/MIT))
