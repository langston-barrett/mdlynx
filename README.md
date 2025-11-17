# mdlynx

Small, fast tool to find broken file links in Markdown documents.

## Features

- Checks whether Markdown file link targets exist
- Precompiled binaries available
- Fast
- Friendly error messages
- Safe, doesn't panic
- 61 lines of code
- [GitHub action](./.github/actions/install-mdlynx/action.yml)

## Non-Features

- No recursive mode, use `find`/`xargs`/shell globs
- Not parallel, use `xargs`, `make`, or `ninja`
- Doesn't touch the network, doesn't check for broken web links

## Install

Download a binary from the [releases page][releases], or build with
[Cargo][cargo]:

```sh
cargo install mdlynx
```

[cargo]: https://doc.rust-lang.org/cargo/
[releases]: https://github.com/langston-barrett/mdlynx/releases
