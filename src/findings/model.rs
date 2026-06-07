use super::severity::Severity;
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindingStatus {
    Open,
    InProgress,
    Remediated,
    AcceptedRisk,
    Closed,
}

impl std::fmt::Display for FindingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Open => "Open",
            Self::InProgress => "In Progress",
            Self::Remediated => "Remediated",
            Self::AcceptedRisk => "Accepted Risk",
            Self::Closed => "Closed",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Observed,
    Confirmed,
    Inferred,
    NeedsValidation,
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Observed => "Observed",
            Self::Confirmed => "Confirmed",
            Self::Inferred => "Inferred",
            Self::NeedsValidation => "Needs Validation",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackMapping {
    pub tactic: String,
    pub technique: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingNote {
    pub text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub severity: Option<Severity>,
    pub status: FindingStatus,
    pub asset_ids: Vec<String>,
    pub summary: Option<String>,
    pub impact: Option<String>,
    pub recommendation: Option<String>,
    pub evidence_ids: Vec<String>,
    pub attack_mappings: Vec<AttackMapping>,
    pub owasp_mappings: Vec<String>,
    pub csf_mappings: Vec<String>,
    pub confidence: Option<Confidence>,
    pub notes: Vec<FindingNote>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
