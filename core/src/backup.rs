use anyhow::{bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackupEntry {
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackupFile {
    pub path: PathBuf,
    pub file_name: String,
    pub size_bytes: u64,
    pub modified_at: Option<DateTime<Utc>>,
}

pub fn default_backup_dir(map_path: &Path) -> PathBuf {
    map_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(".quaver-qol-backups")
}

pub fn create_backup(path: impl AsRef<Path>, backup_dir: Option<&Path>) -> Result<BackupEntry> {
    let path = path.as_ref();
    if !path.exists() {
        bail!("cannot back up missing file: {}", path.display());
    }

    let created_at = Utc::now();
    let backup_dir = backup_dir
        .map(Path::to_path_buf)
        .unwrap_or_else(|| default_backup_dir(path));
    fs::create_dir_all(&backup_dir)
        .with_context(|| format!("failed to create {}", backup_dir.display()))?;

    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("map");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("qua");
    let stamp = created_at.format("%Y%m%dT%H%M%S%.3fZ");
    let backup_path = backup_dir.join(format!("{stem}.{stamp}.{ext}.bak"));
    fs::copy(path, &backup_path).with_context(|| {
        format!(
            "failed to copy {} to {}",
            path.display(),
            backup_path.display()
        )
    })?;

    Ok(BackupEntry {
        original_path: path.to_path_buf(),
        backup_path,
        created_at,
    })
}

pub fn restore_backup(backup_path: impl AsRef<Path>, target_path: impl AsRef<Path>) -> Result<()> {
    let backup_path = backup_path.as_ref();
    let target_path = target_path.as_ref();

    if !backup_path.exists() {
        bail!("backup does not exist: {}", backup_path.display());
    }

    create_backup(target_path, None)?;
    fs::copy(backup_path, target_path).with_context(|| {
        format!(
            "failed to restore {} to {}",
            backup_path.display(),
            target_path.display()
        )
    })?;
    Ok(())
}

pub fn list_backups(map_path: impl AsRef<Path>) -> Result<Vec<BackupFile>> {
    let map_path = map_path.as_ref();
    let backup_dir = default_backup_dir(map_path);
    if !backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut backups = Vec::new();
    for entry in fs::read_dir(&backup_dir)
        .with_context(|| format!("failed to read {}", backup_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(file_name) = path
            .file_name()
            .and_then(|s| s.to_str())
            .map(str::to_string)
        else {
            continue;
        };

        let metadata = entry.metadata()?;
        let modified_at = metadata.modified().ok().map(DateTime::<Utc>::from);
        backups.push(BackupFile {
            path,
            file_name,
            size_bytes: metadata.len(),
            modified_at,
        });
    }

    backups.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(backups)
}
