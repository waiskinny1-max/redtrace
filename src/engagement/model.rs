use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngagementStatus {
    Active,
    Paused,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engagement {
    pub id: String,
    pub name: String,
    pub client: Option<String>,
    pub status: EngagementStatus,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub rules_of_engagement: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Engagement {
    pub fn new(name: String, client: Option<String>, rules_of_engagement: Option<String>) -> Self {
        Self {
            id: format!("ENG-{}", Uuid::new_v4().simple()),
            name,
            client,
            status: EngagementStatus::Active,
            start_date: None,
            end_date: None,
            rules_of_engagement,
            created_at: Utc::now(),
        }
    }
}
