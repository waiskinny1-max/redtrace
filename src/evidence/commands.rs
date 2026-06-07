use super::hashing::sha256_file;
use super::model::Evidence;
use super::vault::{evidence_dir, metadata_path};
use crate::workspace::{next_sequential_id, read_yaml, require_workspace, write_yaml};
use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_all() -> Result<Vec<Evidence>> {
    let dir = evidence_dir()?;
    let mut evidence = vec![];
    if !dir.exists() {
        return Ok(evidence);
    }
    for entry in fs::read_dir(&dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let metadata = entry.path().join("metadata.yaml");
        if metadata.exists() {
            evidence.push(read_yaml(&metadata)?);
        }
    }
    evidence.sort_by(|a: &Evidence, b: &Evidence| a.id.cmp(&b.id));
    Ok(evidence)
}

pub fn load(id: &str) -> Result<Evidence> {
    let path = metadata_path(id)?;
    if !path.exists() {
        bail!("evidence not found: {id}");
    }
    read_yaml(&path)
}

pub fn save(evidence: &Evidence) -> Result<()> {
    write_yaml(&metadata_path(&evidence.id)?, evidence)
}

pub fn add(
    source: PathBuf,
    finding_id: Option<String>,
    asset_id: Option<String>,
    evidence_type: Option<String>,
    note: Option<String>,
) -> Result<()> {
    if !source.is_file() {
        bail!("evidence source is not a file: {}", source.display());
    }

    let existing = load_all()?;
    let id = next_sequential_id("EV", existing.into_iter().map(|evidence| evidence.id));
    let original_filename = source
        .file_name()
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "original.bin".to_string());

    let vault_dir = evidence_dir()?.join(&id);
    fs::create_dir_all(&vault_dir)?;
    let stored_filename = if Path::new(&original_filename).extension().is_some() {
        format!("original.{}", Path::new(&original_filename).extension().unwrap().to_string_lossy())
    } else {
        "original.bin".to_string()
    };
    let stored_absolute = vault_dir.join(&stored_filename);
    fs::copy(&source, &stored_absolute).with_context(|| format!("failed to copy {}", source.display()))?;

    let hash = sha256_file(&stored_absolute)?;
    let stored_relative = PathBuf::from("evidence").join(&id).join(&stored_filename);
    let evidence_type = evidence_type.unwrap_or_else(|| infer_type(&original_filename));

    let evidence = Evidence {
        id: id.clone(),
        finding_id: finding_id.clone(),
        asset_id,
        evidence_type,
        original_filename,
        stored_path: stored_relative,
        sha256: hash,
        operator_note: note,
        created_at: Utc::now(),
    };
    save(&evidence)?;

    if let Some(finding_id) = finding_id {
        crate::findings::commands::attach_evidence(&finding_id, &id)?;
    }

    println!("added evidence {id}");
    Ok(())
}

fn infer_type(filename: &str) -> String {
    let lower = filename.to_ascii_lowercase();
    if lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "screenshot".to_string()
    } else if lower.ends_with(".txt") || lower.ends_with(".log") {
        "terminal-output".to_string()
    } else {
        "file".to_string()
    }
}

pub fn list() -> Result<()> {
    for evidence in load_all()? {
        println!("{} {} {}", evidence.id, evidence.evidence_type, evidence.original_filename);
    }
    Ok(())
}

pub fn show(id: String) -> Result<()> {
    let evidence = load(&id)?;
    println!("{} — {}", evidence.id, evidence.original_filename);
    println!("type: {}", evidence.evidence_type);
    println!("stored path: {}", evidence.stored_path.display());
    println!("sha256: {}", evidence.sha256);
    println!("finding: {}", evidence.finding_id.unwrap_or_else(|| "n/a".to_string()));
    println!("asset: {}", evidence.asset_id.unwrap_or_else(|| "n/a".to_string()));
    if let Some(note) = evidence.operator_note {
        println!("note: {note}");
    }
    Ok(())
}

pub fn verify(id: String) -> Result<bool> {
    let evidence = load(&id)?;
    verify_loaded(&evidence)
}

pub fn verify_all() -> Result<()> {
    let mut failed = 0usize;
    for evidence in load_all()? {
        if !verify_loaded(&evidence)? {
            failed += 1;
        }
    }
    if failed > 0 {
        bail!("{failed} evidence file(s) failed verification");
    }
    Ok(())
}


pub fn chain(out: Option<PathBuf>) -> Result<()> {
    let evidence = load_all()?;
    let mut markdown = String::new();
    let mut failed = 0usize;

    use std::fmt::Write as _;
    writeln!(markdown, "# Evidence Chain of Custody")?;
    writeln!(markdown)?;
    writeln!(markdown, "| ID | Status | Original Filename | Type | Finding | Asset | SHA-256 | Stored Path |")?;
    writeln!(markdown, "|---|---|---|---|---|---|---|---|")?;

    println!("Evidence Chain");
    println!();

    for item in &evidence {
        let (ok, current_hash) = verification_snapshot(item)?;
        let status = if ok {
            "OK"
        } else {
            failed += 1;
            "FAILED"
        };
        println!(
            "{:<7} {:<7} {:<28} finding={} asset={}",
            item.id,
            status,
            item.original_filename,
            item.finding_id.as_deref().unwrap_or("-"),
            item.asset_id.as_deref().unwrap_or("-")
        );

        writeln!(
            markdown,
            "| {} | {} | {} | {} | {} | {} | `{}` | `{}` |",
            item.id,
            status,
            item.original_filename,
            item.evidence_type,
            item.finding_id.as_deref().unwrap_or("-"),
            item.asset_id.as_deref().unwrap_or("-"),
            item.sha256,
            item.stored_path.display()
        )?;

        if !ok {
            writeln!(markdown)?;
            writeln!(markdown, "> Hash mismatch for {}: stored `{}`, current `{}`", item.id, item.sha256, current_hash)?;
            writeln!(markdown)?;
        }
    }

    if let Some(out) = out {
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&out, markdown)?;
        println!();
        println!("wrote chain-of-custody report to {}", out.display());
    }

    println!();
    println!("evidence files: {}", evidence.len());
    println!("failed verification: {failed}");
    Ok(())
}

fn verification_snapshot(evidence: &Evidence) -> Result<(bool, String)> {
    let current_path = require_workspace()?.join(&evidence.stored_path);
    let current_hash = sha256_file(&current_path)?;
    Ok((current_hash == evidence.sha256, current_hash))
}


pub fn verify_loaded(evidence: &Evidence) -> Result<bool> {
    let current_path = require_workspace()?.join(&evidence.stored_path);
    let current_hash = sha256_file(&current_path)?;
    let ok = current_hash == evidence.sha256;
    let status = if ok { "OK" } else { "FAILED" };
    println!("{} {} {}", evidence.id, status, evidence.original_filename);
    println!("stored:  {}", evidence.sha256);
    println!("current: {}", current_hash);
    Ok(ok)
}
