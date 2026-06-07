use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub finding_id: Option<String>,
    pub asset_id: Option<String>,
    pub evidence_type: String,
    pub original_filename: String,
    pub stored_path: PathBuf,
    pub sha256: String,
    pub operator_note: Option<String>,
    pub created_at: DateTime<Utc>,
}
