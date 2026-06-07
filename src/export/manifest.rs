use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::fmt::Write as _;

#[derive(Debug, Serialize)]
pub struct ExportMetadata {
    pub tool: String,
    pub export_format: String,
    pub exported_at: chrono::DateTime<Utc>,
    pub engagement: String,
    pub client: Option<String>,
    pub findings: usize,
    pub evidence_items: usize,
    pub assets: usize,
    pub timeline_events: usize,
    pub validation_status: String,
}

pub fn build_metadata() -> Result<ExportMetadata> {
    let engagement = crate::engagement::storage::load_engagement()?;
    let findings = crate::findings::commands::load_all()?;
    let evidence = crate::evidence::commands::load_all()?;
    let assets = crate::assets::commands::load_store()?;
    let timeline = crate::timeline::commands::load_all()?;

    Ok(ExportMetadata {
        tool: "redtrace".to_string(),
        export_format: "zip".to_string(),
        exported_at: Utc::now(),
        engagement: engagement.name,
        client: engagement.client,
        findings: findings.len(),
        evidence_items: evidence.len(),
        assets: assets.assets.len(),
        timeline_events: timeline.len(),
        validation_status: infer_validation_status()?,
    })
}

pub fn build_hash_manifest() -> Result<String> {
    let workspace = crate::workspace::require_workspace()?;
    let evidence = crate::evidence::commands::load_all()?;
    let mut manifest = String::new();

    writeln!(manifest, "# redtrace evidence hash manifest")?;
    writeln!(manifest, "# generated_at: {}", Utc::now())?;
    writeln!(manifest)?;

    if evidence.is_empty() {
        writeln!(manifest, "No evidence items recorded.")?;
        return Ok(manifest);
    }

    for item in evidence {
        let current_path = workspace.join(&item.stored_path);
        let current_hash = crate::evidence::hashing::sha256_file(&current_path)?;
        let status = if current_hash == item.sha256 { "OK" } else { "FAILED" };
        writeln!(
            manifest,
            "{}\t{}\t{}\tsha256:{}\tpath:{}",
            item.id,
            status,
            item.original_filename,
            item.sha256,
            item.stored_path.display()
        )?;
    }

    Ok(manifest)
}

fn infer_validation_status() -> Result<String> {
    let workspace = crate::workspace::require_workspace()?;
    let assets = crate::assets::commands::load_store()?;
    let findings = crate::findings::commands::load_all()?;
    let evidence = crate::evidence::commands::load_all()?;

    let mut blocking = false;
    let mut warnings = false;

    for asset in assets.assets {
        match asset.scope_status {
            crate::scope::model::ScopeCheckStatus::OutOfScope | crate::scope::model::ScopeCheckStatus::Excluded => blocking = true,
            crate::scope::model::ScopeCheckStatus::Unknown => warnings = true,
            crate::scope::model::ScopeCheckStatus::InScope => {}
        }
    }

    for finding in findings {
        if finding.severity.is_none() || finding.recommendation.as_ref().map_or(true, |value| value.trim().is_empty()) {
            blocking = true;
        }
        if finding.evidence_ids.is_empty()
            || finding.asset_ids.is_empty()
            || finding.confidence.is_none()
            || finding.summary.as_ref().map_or(true, |value| value.trim().is_empty())
            || finding.impact.as_ref().map_or(true, |value| value.trim().is_empty())
        {
            warnings = true;
        }
    }

    for item in evidence {
        let current_path = workspace.join(&item.stored_path);
        match crate::evidence::hashing::sha256_file(&current_path) {
            Ok(current_hash) if current_hash == item.sha256 => {}
            Ok(_) | Err(_) => blocking = true,
        }
    }

    let status = if blocking {
        "failed"
    } else if warnings {
        "warnings"
    } else {
        "passed"
    };

    Ok(status.to_string())
}
