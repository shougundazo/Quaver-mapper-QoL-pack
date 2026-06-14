use quaver_qol_core::{
    apply_macro, check_map, create_backup, list_backups, load_bookmark_sidecar, load_qua,
    merge_bookmark_extensions, resnap_map, restore_backup, write_qua, MacroKind, MacroOptions,
    ResnapOptions, WriteOptions,
};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResnapPayload {
    path: PathBuf,
    snap_divisor: i32,
    max_offset_ms: i32,
    include_long_note_ends: bool,
    write: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MacroPayload {
    path: PathBuf,
    kind: MacroKind,
    start_time: Option<i32>,
    end_time: Option<i32>,
    lane: Option<i32>,
    amount_ms: Option<i32>,
    write: bool,
}

#[tauri::command]
fn load_map_summary(path: PathBuf) -> Result<serde_json::Value, String> {
    let map = load_qua(&path).map_err(to_string)?;
    Ok(json!({
        "path": path,
        "title": map.title,
        "artist": map.artist,
        "difficultyName": map.difficulty_name,
        "mode": map.mode,
        "notes": map.hit_objects.len(),
        "timingPoints": map.timing_points.len(),
        "scrollVelocities": map.slider_velocities.len(),
        "bookmarks": map.bookmarks.len()
    }))
}

#[tauri::command]
fn run_checker(path: PathBuf) -> Result<serde_json::Value, String> {
    let map = load_qua(path).map_err(to_string)?;
    serde_json::to_value(check_map(&map)).map_err(to_string)
}

#[tauri::command]
fn run_resnap(payload: ResnapPayload) -> Result<serde_json::Value, String> {
    let mut map = load_qua(&payload.path).map_err(to_string)?;
    let report = resnap_map(
        &mut map,
        &ResnapOptions {
            snap_divisor: payload.snap_divisor,
            max_offset_ms: payload.max_offset_ms,
            include_long_note_ends: payload.include_long_note_ends,
        },
    );
    let diff = write_qua(
        &payload.path,
        &map,
        &WriteOptions {
            dry_run: !payload.write,
            backup_dir: None,
        },
    )
    .map_err(to_string)?;
    Ok(json!({ "report": report, "diff": diff, "written": payload.write }))
}

#[tauri::command]
fn run_macro(payload: MacroPayload) -> Result<serde_json::Value, String> {
    let mut map = load_qua(&payload.path).map_err(to_string)?;
    let report = apply_macro(
        &mut map,
        &MacroOptions {
            kind: payload.kind,
            start_time: payload.start_time,
            end_time: payload.end_time,
            lane: payload.lane,
            amount_ms: payload.amount_ms,
        },
    );
    let diff = write_qua(
        &payload.path,
        &map,
        &WriteOptions {
            dry_run: !payload.write,
            backup_dir: None,
        },
    )
    .map_err(to_string)?;
    Ok(json!({ "report": report, "diff": diff, "written": payload.write }))
}

#[tauri::command]
fn create_map_backup(path: PathBuf) -> Result<serde_json::Value, String> {
    serde_json::to_value(create_backup(path, None).map_err(to_string)?).map_err(to_string)
}

#[tauri::command]
fn list_map_backups(path: PathBuf) -> Result<serde_json::Value, String> {
    serde_json::to_value(list_backups(path).map_err(to_string)?).map_err(to_string)
}

#[tauri::command]
fn restore_map_backup(
    backup_path: PathBuf,
    target_path: PathBuf,
) -> Result<serde_json::Value, String> {
    restore_backup(backup_path, target_path).map_err(to_string)?;
    Ok(json!({ "restored": true }))
}

#[tauri::command]
fn load_bookmarks(path: PathBuf, sidecar: PathBuf) -> Result<serde_json::Value, String> {
    let map = load_qua(path).map_err(to_string)?;
    let extensions = load_bookmark_sidecar(sidecar).map_err(to_string)?;
    let merged = merge_bookmark_extensions(&map, &extensions);
    Ok(json!({ "qua": map.bookmarks, "extensions": merged }))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_map_summary,
            run_checker,
            run_resnap,
            run_macro,
            create_map_backup,
            list_map_backups,
            restore_map_backup,
            load_bookmarks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn to_string(error: impl std::fmt::Display) -> String {
    error.to_string()
}
