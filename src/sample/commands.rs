use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

pub fn create(path: PathBuf, force: bool) -> Result<()> {
    if path.exists() {
        if !force {
            bail!("sample path already exists: {}. Use --force to replace it.", path.display());
        }
        fs::remove_dir_all(&path).with_context(|| format!("failed to remove {}", path.display()))?;
    }

    fs::create_dir_all(path.join(".redtrace/findings"))?;
    fs::create_dir_all(path.join(".redtrace/evidence/EV-001"))?;
    fs::create_dir_all(path.join(".redtrace/reports"))?;
    fs::create_dir_all(path.join(".redtrace/mappings"))?;

    write_sample_workspace(&path)?;
    write_sample_reports(&path)?;
    write_sample_readme(&path)?;

    println!("created sample engagement at {}", path.display());
    println!("next:");
    println!("  cd {}", path.display());
    println!("  redtrace status");
    println!("  redtrace validate --strict");
    println!("  redtrace export --format zip");
    Ok(())
}

fn write_sample_workspace(path: &Path) -> Result<()> {
    let root = path.join(".redtrace");
    let now = Utc::now();

    let engagement = crate::engagement::model::Engagement {
        id: "ENG-demo".to_string(),
        name: "ACME Demo Internal Assessment".to_string(),
        client: Some("ACME Demo Corp".to_string()),
        status: crate::engagement::model::EngagementStatus::Active,
        start_date: Some(now),
        end_date: None,
        rules_of_engagement: Some("Authorized lab-only assessment covering 10.10.0.0/24 and *.lab.local. No exploitation, persistence, phishing, or credential collection is performed by redtrace.".to_string()),
        created_at: now,
    };
    crate::workspace::write_yaml(&root.join("engagement.yaml"), &engagement)?;

    let scope = crate::scope::model::ScopeStore {
        rules: vec![
            crate::scope::model::ScopeRule {
                id: "SCOPE-001".to_string(),
                rule_type: crate::scope::model::ScopeRuleType::Cidr,
                value: "10.10.0.0/24".to_string(),
                label: Some("internal-lab".to_string()),
                status: crate::scope::model::ScopeStatus::InScope,
                notes: Some("Authorized internal lab range.".to_string()),
                created_at: now,
            },
            crate::scope::model::ScopeRule {
                id: "SCOPE-002".to_string(),
                rule_type: crate::scope::model::ScopeRuleType::WildcardDomain,
                value: "*.lab.local".to_string(),
                label: Some("lab-domains".to_string()),
                status: crate::scope::model::ScopeStatus::InScope,
                notes: Some("Authorized lab DNS namespace.".to_string()),
                created_at: now,
            },
        ],
        exclusions: vec![crate::scope::model::ScopeExclusion {
            id: "EXCL-001".to_string(),
            rule_type: crate::scope::model::ScopeRuleType::Ip,
            value: "10.10.0.50".to_string(),
            reason: "Database host excluded by rules of engagement.".to_string(),
            created_at: now,
        }],
    };
    crate::workspace::write_yaml(&root.join("scope.yaml"), &scope)?;

    let assets = crate::assets::model::AssetStore {
        assets: vec![
            crate::assets::model::Asset {
                id: "A-001".to_string(),
                hostname: "web01.lab.local".to_string(),
                ip: Some("10.10.0.20".to_string()),
                asset_type: "web".to_string(),
                environment: Some("lab".to_string()),
                tags: vec!["web".to_string(), "auth".to_string()],
                scope_status: crate::scope::model::ScopeCheckStatus::InScope,
                notes: vec![crate::assets::model::AssetNote { text: "Demo web application with administrative route.".to_string(), created_at: now }],
                created_at: now,
                updated_at: now,
            },
            crate::assets::model::Asset {
                id: "A-002".to_string(),
                hostname: "auth.lab.local".to_string(),
                ip: Some("10.10.0.21".to_string()),
                asset_type: "identity".to_string(),
                environment: Some("lab".to_string()),
                tags: vec!["identity".to_string()],
                scope_status: crate::scope::model::ScopeCheckStatus::InScope,
                notes: vec![],
                created_at: now,
                updated_at: now,
            },
        ],
    };
    crate::workspace::write_yaml(&root.join("assets.yaml"), &assets)?;

    let evidence_file = root.join("evidence/EV-001/original.txt");
    fs::write(&evidence_file, "Demo evidence: authorized lab observation for F-001.\n")?;
    let evidence_hash = crate::evidence::hashing::sha256_file(&evidence_file)?;

    let evidence = crate::evidence::model::Evidence {
        id: "EV-001".to_string(),
        finding_id: Some("F-001".to_string()),
        asset_id: Some("A-001".to_string()),
        evidence_type: "terminal-output".to_string(),
        original_filename: "demo-evidence.txt".to_string(),
        stored_path: PathBuf::from("evidence/EV-001/original.txt"),
        sha256: evidence_hash,
        operator_note: Some("Safe demo evidence generated by redtrace sample create.".to_string()),
        created_at: now,
    };
    crate::workspace::write_yaml(&root.join("evidence/EV-001/metadata.yaml"), &evidence)?;

    let finding = crate::findings::model::Finding {
        id: "F-001".to_string(),
        title: "Weak access control on admin endpoint".to_string(),
        severity: Some(crate::findings::severity::Severity::High),
        status: crate::findings::model::FindingStatus::Open,
        asset_ids: vec!["A-001".to_string()],
        summary: Some("The demo application allowed access to an administrative function without enforcing the expected server-side authorization control.".to_string()),
        impact: Some("A low-privileged lab user could access functionality intended for administrators.".to_string()),
        recommendation: Some("Enforce server-side authorization checks on privileged routes and validate authorization decisions independently of client-side state.".to_string()),
        evidence_ids: vec!["EV-001".to_string()],
        attack_mappings: vec![crate::findings::model::AttackMapping { tactic: "TA0003".to_string(), technique: "T1078".to_string() }],
        owasp_mappings: vec!["WSTG-v42-ATHZ-01".to_string()],
        csf_mappings: vec!["protect".to_string()],
        confidence: Some(crate::findings::model::Confidence::Confirmed),
        notes: vec![],
        created_at: now,
        updated_at: now,
    };
    crate::workspace::write_yaml(&root.join("findings/F-001.yaml"), &finding)?;

    let timeline = vec![
        crate::timeline::model::TimelineEvent { time: now, event: "Engagement initialized".to_string(), ref_id: None },
        crate::timeline::model::TimelineEvent { time: now, event: "Scope loaded for internal lab".to_string(), ref_id: Some("SCOPE-001".to_string()) },
        crate::timeline::model::TimelineEvent { time: now, event: "Finding F-001 documented".to_string(), ref_id: Some("F-001".to_string()) },
        crate::timeline::model::TimelineEvent { time: now, event: "Evidence EV-001 added and hashed".to_string(), ref_id: Some("EV-001".to_string()) },
    ];
    let mut timeline_body = String::new();
    for event in timeline {
        timeline_body.push_str(&serde_json::to_string(&event)?);
        timeline_body.push('\n');
    }
    fs::write(root.join("timeline.jsonl"), timeline_body)?;

    Ok(())
}

fn write_sample_reports(path: &Path) -> Result<()> {
    let report = r#"# ACME Demo Internal Assessment

**Report profile:** full  
**Generated by:** redtrace sample create

## Executive Summary

This safe sample engagement demonstrates redtrace's workflow for scope, assets, findings, evidence integrity, mappings, timeline, and reporting.

## Findings Overview

| ID | Title | Severity | Status | Evidence |
|---|---|---|---|---:|
| F-001 | Weak access control on admin endpoint | High | Open | 1 |

## Evidence Integrity

The included demo evidence item is stored locally and hashed with SHA-256.
"#;
    fs::write(path.join(".redtrace/reports/report-full.md"), report)?;

    let html = r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>redtrace sample report</title></head>
<body><h1>ACME Demo Internal Assessment</h1><p>Safe redtrace sample report.</p></body>
</html>
"#;
    fs::write(path.join(".redtrace/reports/report-full.html"), html)?;
    Ok(())
}

fn write_sample_readme(path: &Path) -> Result<()> {
    let readme = r#"# redtrace sample engagement

This directory is a safe demonstration workspace generated by `redtrace sample create`.

Try:

```bash
redtrace status
redtrace validate --strict
redtrace evidence verify-all
redtrace evidence chain
redtrace report --format markdown --profile full
redtrace export --format zip
```

The sample uses lab-only names and addresses. It contains no exploit payloads, phishing material, credential capture, persistence, or scanning logic.
"#;
    fs::write(path.join("README.md"), readme)?;
    Ok(())
}
