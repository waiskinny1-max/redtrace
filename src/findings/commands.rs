use super::model::{Confidence, Finding, FindingNote, FindingStatus};
use super::severity::Severity;
use crate::workspace::{next_sequential_id, read_yaml, require_workspace, write_yaml};
use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

fn findings_dir() -> Result<PathBuf> {
    Ok(require_workspace()?.join("findings"))
}

fn finding_path(id: &str) -> Result<PathBuf> {
    Ok(findings_dir()?.join(format!("{id}.yaml")))
}

pub fn load_all() -> Result<Vec<Finding>> {
    let dir = findings_dir()?;
    let mut findings = vec![];
    if !dir.exists() {
        return Ok(findings);
    }
    for entry in fs::read_dir(&dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|extension| extension == std::ffi::OsStr::new("yaml")) {
            findings.push(read_yaml(&path)?);
        }
    }
    findings.sort_by(|a: &Finding, b: &Finding| a.id.cmp(&b.id));
    Ok(findings)
}

pub fn load(id: &str) -> Result<Finding> {
    let path = finding_path(id)?;
    if !path.exists() {
        bail!("finding not found: {id}");
    }
    read_yaml(&path)
}

pub fn save(finding: &Finding) -> Result<()> {
    write_yaml(&finding_path(&finding.id)?, finding)
}

pub fn new(title: String) -> Result<()> {
    let existing = load_all()?;
    let id = next_sequential_id("F", existing.into_iter().map(|finding| finding.id));
    let now = Utc::now();
    let finding = Finding {
        id: id.clone(),
        title,
        severity: None,
        status: FindingStatus::Open,
        asset_ids: vec![],
        summary: None,
        impact: None,
        recommendation: None,
        evidence_ids: vec![],
        attack_mappings: vec![],
        owasp_mappings: vec![],
        csf_mappings: vec![],
        confidence: None,
        notes: vec![],
        created_at: now,
        updated_at: now,
    };
    save(&finding)?;
    println!("created finding {id}");
    Ok(())
}

pub fn list() -> Result<()> {
    for finding in load_all()? {
        let severity = finding.severity.map(|severity| severity.to_string()).unwrap_or_else(|| "Unrated".to_string());
        println!("{} [{}] {} — {}", finding.id, severity, finding.status, finding.title);
    }
    Ok(())
}

pub fn show(id: String) -> Result<()> {
    let finding = load(&id)?;
    println!("{} — {}", finding.id, finding.title);
    println!("severity: {}", finding.severity.map(|severity| severity.to_string()).unwrap_or_else(|| "unrated".to_string()));
    println!("status: {}", finding.status);
    println!("confidence: {}", finding.confidence.map(|confidence| confidence.to_string()).unwrap_or_else(|| "unset".to_string()));
    println!("assets: {}", finding.asset_ids.join(", "));
    println!("evidence: {}", finding.evidence_ids.join(", "));
    if let Some(summary) = finding.summary {
        println!("summary: {summary}");
    }
    if let Some(impact) = finding.impact {
        println!("impact: {impact}");
    }
    if let Some(recommendation) = finding.recommendation {
        println!("recommendation: {recommendation}");
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn set(
    id: String,
    severity: Option<Severity>,
    asset: Option<String>,
    status: Option<FindingStatus>,
    summary: Option<String>,
    impact: Option<String>,
    recommendation: Option<String>,
    confidence: Option<Confidence>,
) -> Result<()> {
    let mut finding = load(&id)?;
    if let Some(severity) = severity {
        finding.severity = Some(severity);
    }
    if let Some(asset) = asset {
        if !finding.asset_ids.contains(&asset) {
            finding.asset_ids.push(asset);
        }
    }
    if let Some(status) = status {
        finding.status = status;
    }
    if summary.is_some() {
        finding.summary = summary;
    }
    if impact.is_some() {
        finding.impact = impact;
    }
    if recommendation.is_some() {
        finding.recommendation = recommendation;
    }
    if let Some(confidence) = confidence {
        finding.confidence = Some(confidence);
    }
    finding.updated_at = Utc::now();
    save(&finding)?;
    println!("updated {id}");
    Ok(())
}

pub fn note(id: String, text: String) -> Result<()> {
    let mut finding = load(&id)?;
    finding.notes.push(FindingNote { text, created_at: Utc::now() });
    finding.updated_at = Utc::now();
    save(&finding)?;
    println!("added note to {id}");
    Ok(())
}

pub fn close(id: String, status: FindingStatus) -> Result<()> {
    let mut finding = load(&id)?;
    finding.status = status;
    finding.updated_at = Utc::now();
    save(&finding)?;
    println!("closed {id} as {}", finding.status);
    Ok(())
}

pub fn attach_evidence(finding_id: &str, evidence_id: &str) -> Result<()> {
    let mut finding = load(finding_id)?;
    let evidence_id = evidence_id.to_string();
    if !finding.evidence_ids.contains(&evidence_id) {
        finding.evidence_ids.push(evidence_id);
        finding.updated_at = Utc::now();
        save(&finding)?;
    }
    Ok(())
}
