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

**First of all**, install the binary [skim](https://github.com/lotabout/skim#installation).

```bash
$ cargo install conceal
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

[![demo](https://asciinema.org/a/522271.svg)](https://asciinema.org/a/522271?t=7)

**`Tab` is the multi-selection and multi-cancel key !!**



## Unpredictable behavior

**conceal** would **panic** when **restoring** or **deleting** a broken symlink because of the implementation of **trash-rs**.

**conceal** only do the basic checks, which happen before the restoring or emptying process. If a symlink break during the process, **conceal** would panic inevitably.



## License

The MIT License ([MIT](https://opensource.org/licenses/MIT))
