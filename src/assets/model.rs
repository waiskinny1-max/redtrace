use crate::scope::model::ScopeCheckStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetNote {
    pub text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub hostname: String,
    pub ip: Option<String>,
    pub asset_type: String,
    pub environment: Option<String>,
    pub tags: Vec<String>,
    pub scope_status: ScopeCheckStatus,
    pub notes: Vec<AssetNote>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetStore {
    pub assets: Vec<Asset>,
}
