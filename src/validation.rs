use anyhow::Result;

pub fn run() -> Result<()> {
    crate::workspace::require_workspace()?;
    let engagement = crate::engagement::storage::load_engagement()?;
    let assets = crate::assets::commands::load_store()?;
    let findings = crate::findings::commands::load_all()?;
    let evidence = crate::evidence::commands::load_all()?;

    let mut warnings = vec![];

    if engagement.rules_of_engagement.as_ref().map_or(true, |roe| roe.trim().is_empty()) {
        warnings.push("Engagement has no rules of engagement documented.".to_string());
    }

    for asset in assets.assets {
        match asset.scope_status {
            crate::scope::model::ScopeCheckStatus::OutOfScope => {
                warnings.push(format!("Asset {} is marked out-of-scope.", asset.id));
            }
            crate::scope::model::ScopeCheckStatus::Excluded => {
                warnings.push(format!("Asset {} is explicitly excluded by scope rules.", asset.id));
            }
            crate::scope::model::ScopeCheckStatus::Unknown => {
                warnings.push(format!("Asset {} has unknown scope status.", asset.id));
            }
            crate::scope::model::ScopeCheckStatus::InScope => {}
        }
    }

    for finding in findings {
        if finding.severity.is_none() {
            warnings.push(format!("Finding {} has no severity.", finding.id));
        }
        if finding.confidence.is_none() {
            warnings.push(format!("Finding {} has no confidence level.", finding.id));
        }
        if finding.summary.as_ref().map_or(true, |value| value.trim().is_empty()) {
            warnings.push(format!("Finding {} has no summary.", finding.id));
        }
        if finding.impact.as_ref().map_or(true, |value| value.trim().is_empty()) {
            warnings.push(format!("Finding {} has no impact statement.", finding.id));
        }
        if finding.recommendation.as_ref().map_or(true, |value| value.trim().is_empty()) {
            warnings.push(format!("Finding {} has no recommendation.", finding.id));
        }
        if finding.evidence_ids.is_empty() {
            warnings.push(format!("Finding {} has no linked evidence.", finding.id));
        }
    }

    for item in evidence {
        let current_path = crate::workspace::require_workspace()?.join(&item.stored_path);
        match crate::evidence::hashing::sha256_file(&current_path) {
            Ok(current_hash) if current_hash == item.sha256 => {}
            Ok(_) => warnings.push(format!("Evidence {} hash mismatch.", item.id)),
            Err(err) => warnings.push(format!("Evidence {} could not be verified: {err}", item.id)),
        }
    }

    if warnings.is_empty() {
        println!("redtrace validation: OK");
    } else {
        println!("Validation warnings:");
        for warning in warnings {
            println!("- {warning}");
        }
    }

    Ok(())
}
