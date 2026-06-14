# Roadmap

## MVP

- Rust `.qua` model, parser, serializer, and dry-run diff
- Checker for missing timing, invalid lanes, invalid long notes, duplicate/stacked objects, invalid BPM, and broken timing group/sample references
- Backup before every non-dry-run write
- Resnap helper with snap divisor and max offset threshold
- CLI commands for checker, diff, backup, restore, resnap, macro, and bookmark sidecar orphan marking
- Tauri GUI for file selection, checker, resnap, macro, backups, bookmarks, dry-run output, and logs
- Lua plugin for in-editor current time, selected count, existing bookmark display, and standard bookmark add

## Next

- Better `.qua` round-trip compatibility tests using real maps
- Sidecar bookmark editing UI for color, memo, category, and label extensions
- Backup history browser and restore picker in the GUI
- More selection macros: stream windows, chord filters, lane pattern filters, LN-only filters
- Watch-based bridge from plugin config to desktop app
- Performance benchmarks on large maps

## Later

- osu!mania import model behind a format-neutral trait
- Batch processing for mapsets
- CI with Rust, TypeScript, and packaged Tauri smoke builds
- Optional plugin bridge if Quaver exposes safe process/path APIs in the future
