# Quaver-mapper-QoL-pack

Quaver Mapper QoL Pack is an MVP toolset for speeding up Quaver editor work while keeping the heavy map logic outside the UI.

## Layout

```text
quaver-qol/
  core/        Rust library + CLI
  app/         Tauri + TypeScript/React desktop app
  plugin/      Quaver Lua plugin, dual dev/distribution layout
  docs/        Specification and roadmap
```

## Confirmed MVP Scope

- Core logic is Rust and is callable from both CLI and GUI.
- The desktop app is Tauri + TypeScript/React.
- The Quaver editor plugin is Lua.
- `.qua` `Bookmarks` are the source of truth.
- Bookmark sidecar JSON stores only extra metadata that Quaver bookmarks do not support.
- The Lua plugin does not launch the external app and does not resolve the `.qua` absolute path in MVP. Start the app manually and select the `.qua` file there.
- Watch-based plugin-to-app bridging is reserved for a future version.

## Core Features

- `.qua` load/write
- dry-run diff output
- automatic backup before non-dry-run writes
- checker
- resnap helper
- selection macros
- bookmark sidecar orphan marking
- backup/restore

## CLI

Install Rust, then run:

```powershell
cd quaver-qol
cargo test
cargo run -p quaver-qol-core --bin quaver-qol -- check path\to\map.qua
cargo run -p quaver-qol-core --bin quaver-qol -- resnap path\to\map.qua --snap 4 --max-offset-ms 6
cargo run -p quaver-qol-core --bin quaver-qol -- resnap path\to\map.qua --snap 4 --max-offset-ms 6 --write
```

Commands dry-run by default unless `--write` is provided.

## Desktop App

Install Rust and Node, then run:

```powershell
cd quaver-qol\app
npm.cmd install
npm.cmd run tauri dev
```

`npm.cmd` is used on Windows to avoid PowerShell execution policy issues with `npm.ps1`.

## Quaver Plugin

Development files:

- `plugin/main.lua`
- `plugin/plugin.ini`

Quaver distribution files:

- `plugin/plugin.lua`
- `plugin/settings.ini`

To refresh distribution files after editing the development files:

```powershell
.\plugin\build.ps1
```

Copy a folder containing `plugin.lua` and `settings.ini` into Quaver's `Plugins` directory. The plugin adds a Quaver editor window with current time, selected note count, map counts, existing bookmark display, and an `actions.AddBookmark` helper.

## Bookmark Sidecar

The `.qua` file remains authoritative for actual bookmarks. Sidecar JSON is named by convention:

```text
song.quaver-qol.bookmarks.json
```

Each entry can store:

- `startTime`
- `label`
- `color`
- `memo`
- `category`
- `orphan`

If a sidecar `startTime` has no matching `.qua` bookmark, it is marked `orphan` and the GUI should warn instead of resolving conflicts automatically.

## Environment Notes

This repository expects Rust and Node tooling. If PowerShell blocks `npm`, use `npm.cmd`.
