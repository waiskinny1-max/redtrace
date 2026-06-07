use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub time: DateTime<Utc>,
    pub event: String,
    pub ref_id: Option<String>,
}
