use crate::workspace::require_workspace;
use anyhow::Result;
use std::path::PathBuf;

pub fn evidence_dir() -> Result<PathBuf> {
    Ok(require_workspace()?.join("evidence"))
}

pub fn metadata_path(id: &str) -> Result<PathBuf> {
    Ok(evidence_dir()?.join(id).join("metadata.yaml"))
}
