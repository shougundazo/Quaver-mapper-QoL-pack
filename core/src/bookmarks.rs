use crate::model::{BookmarkExtension, QuaMap};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn load_bookmark_sidecar(path: impl AsRef<Path>) -> Result<Vec<BookmarkExtension>> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))
}

pub fn save_bookmark_sidecar(
    path: impl AsRef<Path>,
    extensions: &[BookmarkExtension],
) -> Result<()> {
    let path = path.as_ref();
    let text =
        serde_json::to_string_pretty(extensions).context("failed to serialize bookmark sidecar")?;
    fs::write(path, text).with_context(|| format!("failed to write {}", path.display()))
}

pub fn merge_bookmark_extensions(
    map: &QuaMap,
    extensions: &[BookmarkExtension],
) -> Vec<BookmarkExtension> {
    let qua_times = map
        .bookmarks
        .iter()
        .map(|b| b.start_time)
        .collect::<HashSet<_>>();

    extensions
        .iter()
        .cloned()
        .map(|mut ext| {
            ext.orphan = !qua_times.contains(&ext.start_time);
            ext
        })
        .collect()
}
