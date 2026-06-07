use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScopeRuleType {
    Cidr,
    Ip,
    Domain,
    WildcardDomain,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScopeStatus {
    InScope,
    Excluded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScopeCheckStatus {
    InScope,
    Excluded,
    OutOfScope,
    Unknown,
}

impl std::fmt::Display for ScopeCheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::InScope => "in-scope",
            Self::Excluded => "excluded",
            Self::OutOfScope => "out-of-scope",
            Self::Unknown => "unknown",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeRule {
    pub id: String,
    pub rule_type: ScopeRuleType,
    pub value: String,
    pub label: Option<String>,
    pub status: ScopeStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeExclusion {
    pub id: String,
    pub rule_type: ScopeRuleType,
    pub value: String,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScopeStore {
    pub rules: Vec<ScopeRule>,
    pub exclusions: Vec<ScopeExclusion>,
}
