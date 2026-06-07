use crate::error::RedtraceError;
use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const WORKSPACE_DIR: &str = ".redtrace";

pub fn workspace_path() -> PathBuf {
    PathBuf::from(WORKSPACE_DIR)
}

pub fn require_workspace() -> Result<PathBuf> {
    let path = workspace_path();
    if !path.is_dir() {
        return Err(RedtraceError::MissingWorkspace.into());
    }
    Ok(path)
}

pub fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;
    }
    Ok(())
}

pub fn read_yaml<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let contents = fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let value = serde_yaml::from_str(&contents).with_context(|| format!("failed to parse YAML {}", path.display()))?;
    Ok(value)
}

pub fn write_yaml<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    ensure_parent(path)?;
    let contents = serde_yaml::to_string(value).context("failed to serialize YAML")?;
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub fn read_yaml_or_default<T: DeserializeOwned + Default>(path: &Path) -> Result<T> {
    if !path.exists() {
        return Ok(T::default());
    }
    read_yaml(path)
}

pub fn next_sequential_id(prefix: &str, used_ids: impl IntoIterator<Item = String>) -> String {
    let mut max_seen = 0u32;
    for id in used_ids {
        if let Some(num) = id.strip_prefix(&format!("{prefix}-")) {
            if let Ok(parsed) = num.parse::<u32>() {
                max_seen = max_seen.max(parsed);
            }
        }
    }
    format!("{prefix}-{:03}", max_seen + 1)
}
