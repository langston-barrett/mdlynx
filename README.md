# mdlynx

Small, fast tool to find broken file links in Markdown documents.

## Features

- Checks whether Markdown file link targets exist
- Statically linked Linux binaries available
- Fast, parallel
- Friendly error messages
- Safe, doesn't panic
- 70 lines of code

## Non-Features

- No recursive mode, use `find`/`xargs`/shell globs
- Doesn't touch the network, doesn't check for broken web links

## Install

Download a binary from the [releases page][releases], or build with
[Cargo][cargo]:

```sh
cargo install mdlynx
```

[cargo]: https://doc.rust-lang.org/cargo/
[releases]: https://github.com/langston-barrett/mdlynx/releases
