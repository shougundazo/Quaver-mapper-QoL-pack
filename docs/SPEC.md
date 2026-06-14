# Quaver Mapper QoL Pack Specification

## Architecture

The MVP is split by responsibility rather than forcing one language everywhere.

- `core`: Rust library and CLI for `.qua` parsing, checking, backup, dry-run diffing, resnap, macros, and bookmark sidecar handling.
- `app`: Tauri + TypeScript/React desktop UI that calls Rust commands and keeps UI state separate from core map logic.
- `plugin`: Lua Quaver editor plugin for in-editor helper UI.
- `tests`: Rust unit/integration tests in `core/tests` and TypeScript tests in `app/src/*.test.ts`.

## Confirmed MVP Decisions

- Quaver plugin external integration is manual for MVP. Users launch the Tauri/CLI app themselves and select the target `.qua` in the app.
- The plugin does not attempt to launch an external process or discover the absolute `.qua` path because those APIs are not guaranteed in Quaver's Lua sandbox.
- Plugin file layout is dual. Development files are `plugin/main.lua` and `plugin/plugin.ini`; Quaver distribution files are `plugin/plugin.lua` and `plugin/settings.ini`.
- Quaver `.qua` `Bookmarks` are the source of truth.
- Sidecar JSON stores only extended bookmark metadata: label, color, memo, category, and orphan state.
- Sidecar entries whose `startTime` does not exist in `.qua` bookmarks are marked `orphan` and surfaced as warnings; conflict resolution UI is not part of MVP.

## Core Interfaces

Rust public API is exposed from `quaver_qol_core`:

- `load_qua`, `parse_qua_str`, `write_qua`, `write_qua_dry_run`
- `check_map`
- `create_backup`, `list_backups`, `restore_backup`
- `resnap_map`
- `apply_macro`
- `load_bookmark_sidecar`, `save_bookmark_sidecar`, `merge_bookmark_extensions`
- `diff_text`

Write behavior:

- `WriteOptions::default()` is dry-run.
- Any non-dry-run write calls `create_backup` before replacing the `.qua`.
- Backups are written next to the map under `.quaver-qol-backups`.

## Data Model

The Rust model covers:

- metadata fields such as title, artist, creator, difficulty, mode, audio, and background
- hit objects, timing points, slider velocities, scroll speed factors
- timing groups and default/global group-compatible fields
- Quaver bookmarks
- custom audio samples and sound effects

Unknown YAML fields are preserved via flattened `extra` maps where practical so future `.qua` fields are less likely to be destroyed by round trips.

## CLI

The CLI binary is `quaver-qol`.

- `check <qua>` prints checker JSON.
- `diff <qua>` prints normalized serialization diff.
- `resnap <qua> --snap <n> --max-offset-ms <n> [--write]` dry-runs unless `--write` is present.
- `macro <qua> --kind <kind> ... [--write]` dry-runs unless `--write` is present.
- `backup <qua>` creates an explicit backup.
- `restore <backup> <qua>` restores a backup after backing up the current target.
- `bookmarks <qua> <sidecar>` marks orphan bookmark extensions.

## GUI

The Tauri UI supports:

- `.qua` selection
- checker result list
- resnap settings and dry-run diff
- macro execution and dry-run diff
- bookmark sidecar loading with orphan semantics
- manual backup creation
- backup history listing
- log output

The app calls Rust commands in `src-tauri` and does not implement map mutation logic in TypeScript.

## Lua Plugin

The Quaver plugin provides:

- current editor time display
- selected note count
- map counts
- current bookmark display
- standard Quaver bookmark add via `actions.AddBookmark`
- a remembered bookmark request in plugin `config.yaml`

It intentionally does not:

- launch the external app
- read or write arbitrary files
- resolve the current `.qua` absolute path
- manage sidecar JSON directly

Watch-based external app integration is reserved for a future release.
