use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IssueLevel {
    Critical,
    Error,
    Warning,
}

impl IssueLevel {
    fn label(self) -> &'static str {
        match self {
            Self::Critical => "CRITICAL",
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
        }
    }

    fn blocks_strict(self) -> bool {
        matches!(self, Self::Critical | Self::Error)
    }
}

#[derive(Debug, Clone)]
struct Issue {
    level: IssueLevel,
    message: String,
}

pub fn run(strict: bool) -> Result<()> {
    let workspace = crate::workspace::require_workspace()?;
    let engagement = crate::engagement::storage::load_engagement()?;
    let assets = crate::assets::commands::load_store()?;
    let findings = crate::findings::commands::load_all()?;
    let evidence = crate::evidence::commands::load_all()?;

    let mut issues = vec![];

    if engagement.rules_of_engagement.as_ref().map_or(true, |roe| roe.trim().is_empty()) {
        issues.push(issue(IssueLevel::Warning, "Engagement has no rules of engagement documented."));
    }

    for asset in assets.assets {
        match asset.scope_status {
            crate::scope::model::ScopeCheckStatus::OutOfScope => {
                issues.push(issue(IssueLevel::Critical, format!("Asset {} is marked out-of-scope.", asset.id)));
            }
            crate::scope::model::ScopeCheckStatus::Excluded => {
                issues.push(issue(IssueLevel::Critical, format!("Asset {} is explicitly excluded by scope rules.", asset.id)));
            }
            crate::scope::model::ScopeCheckStatus::Unknown => {
                issues.push(issue(IssueLevel::Warning, format!("Asset {} has unknown scope status.", asset.id)));
            }
            crate::scope::model::ScopeCheckStatus::InScope => {}
        }
    }

    for finding in findings {
        if finding.severity.is_none() {
            issues.push(issue(IssueLevel::Error, format!("Finding {} has no severity.", finding.id)));
        }
        if finding.confidence.is_none() {
            issues.push(issue(IssueLevel::Warning, format!("Finding {} has no confidence level.", finding.id)));
        }
        if finding.asset_ids.is_empty() {
            issues.push(issue(IssueLevel::Warning, format!("Finding {} has no affected asset.", finding.id)));
        }
        if finding.summary.as_ref().map_or(true, |value| value.trim().is_empty()) {
            issues.push(issue(IssueLevel::Warning, format!("Finding {} has no summary.", finding.id)));
        }
        if finding.impact.as_ref().map_or(true, |value| value.trim().is_empty()) {
            issues.push(issue(IssueLevel::Warning, format!("Finding {} has no impact statement.", finding.id)));
        }
        if finding.recommendation.as_ref().map_or(true, |value| value.trim().is_empty()) {
            issues.push(issue(IssueLevel::Error, format!("Finding {} has no recommendation.", finding.id)));
        }
        if finding.evidence_ids.is_empty() {
            issues.push(issue(IssueLevel::Warning, format!("Finding {} has no linked evidence.", finding.id)));
        }
    }

    for item in evidence {
        let current_path = workspace.join(&item.stored_path);
        match crate::evidence::hashing::sha256_file(&current_path) {
            Ok(current_hash) if current_hash == item.sha256 => {}
            Ok(_) => issues.push(issue(IssueLevel::Critical, format!("Evidence {} hash mismatch.", item.id))),
            Err(err) => issues.push(issue(IssueLevel::Critical, format!("Evidence {} could not be verified: {err}", item.id))),
        }
    }

    print_report(&issues);

    if strict && issues.iter().any(|item| item.level.blocks_strict()) {
        bail!("strict validation failed");
    }

    Ok(())
}

fn issue(level: IssueLevel, message: impl Into<String>) -> Issue {
    Issue { level, message: message.into() }
}

fn print_report(issues: &[Issue]) {
    if issues.is_empty() {
        println!("Validation Report");
        println!();
        println!("Result: OK");
        return;
    }

    println!("Validation Report");
    println!();

    for level in [IssueLevel::Critical, IssueLevel::Error, IssueLevel::Warning] {
        let bucket: Vec<&Issue> = issues.iter().filter(|item| item.level == level).collect();
        if bucket.is_empty() {
            continue;
        }

        println!("{}", level.label());
        for item in bucket {
            println!("  - {}", item.message);
        }
        println!();
    }

    let critical = issues.iter().filter(|item| item.level == IssueLevel::Critical).count();
    let errors = issues.iter().filter(|item| item.level == IssueLevel::Error).count();
    let warnings = issues.iter().filter(|item| item.level == IssueLevel::Warning).count();

    let result = if critical > 0 || errors > 0 { "FAILED" } else { "WARNINGS" };
    println!("Summary: {critical} critical, {errors} errors, {warnings} warnings");
    println!("Result: {result}");
}
