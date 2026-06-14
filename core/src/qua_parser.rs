use crate::backup::create_backup;
use crate::diff::{diff_text, DiffSummary};
use crate::model::QuaMap;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct WriteOptions {
    pub dry_run: bool,
    pub backup_dir: Option<PathBuf>,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            dry_run: true,
            backup_dir: None,
        }
    }
}

pub fn parse_qua_str(input: &str) -> Result<QuaMap> {
    let map = serde_yaml::from_str(input).context("failed to parse .qua YAML")?;
    Ok(map)
}

pub fn load_qua(path: impl AsRef<Path>) -> Result<QuaMap> {
    let path = path.as_ref();
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    parse_qua_str(&content)
}

pub fn serialize_qua(map: &QuaMap) -> Result<String> {
    let mut clone = map.clone();
    clone.sort_for_write();
    let text = serde_yaml::to_string(&clone).context("failed to serialize .qua YAML")?;
    Ok(text.replace('\n', "\r\n"))
}

pub fn write_qua_dry_run(path: impl AsRef<Path>, map: &QuaMap) -> Result<DiffSummary> {
    let path = path.as_ref();
    let before = fs::read_to_string(path).unwrap_or_default();
    let after = serialize_qua(map)?;
    Ok(diff_text(&before, &after))
}

pub fn write_qua(
    path: impl AsRef<Path>,
    map: &QuaMap,
    options: &WriteOptions,
) -> Result<DiffSummary> {
    let path = path.as_ref();
    let before = fs::read_to_string(path).unwrap_or_default();
    let after = serialize_qua(map)?;
    let diff = diff_text(&before, &after);

    if options.dry_run {
        return Ok(diff);
    }

    create_backup(path, options.backup_dir.as_deref())?;
    fs::write(path, after).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(diff)
}
