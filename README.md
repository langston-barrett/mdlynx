# mdlynx

Small, fast tool to find broken file links in Markdown documents.

## Features

- Checks whether Markdown file link targets exist
- Statically compiled binaries available
- Fast, parallel
- Friendly error messages
- Safe, doesn't panic
- 64 lines of code

## Non-Features

- No recursive mode, use `find`/`xargs`/shell globs
- Doesn't touch the network, doesn't check for broken web links