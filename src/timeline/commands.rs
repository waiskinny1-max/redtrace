use super::model::TimelineEvent;
use crate::workspace::require_workspace;
use anyhow::{Context, Result};
use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn timeline_path() -> Result<PathBuf> {
    Ok(require_workspace()?.join("timeline.jsonl"))
}

pub fn add(event: String, ref_id: Option<String>) -> Result<()> {
    let entry = TimelineEvent { time: Utc::now(), event, ref_id };
    let encoded = serde_json::to_string(&entry)?;
    let mut file = OpenOptions::new().create(true).append(true).open(timeline_path()?)?;
    writeln!(file, "{encoded}")?;
    println!("added timeline event");
    Ok(())
}

pub fn load_all() -> Result<Vec<TimelineEvent>> {
    let path = timeline_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let contents = fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut events = vec![];
    for line in contents.lines().filter(|line| !line.trim().is_empty()) {
        events.push(serde_json::from_str(line)?);
    }
    Ok(events)
}

pub fn list() -> Result<()> {
    for entry in load_all()? {
        let reference = entry.ref_id.unwrap_or_else(|| "-".to_string());
        println!("{} [{}] {}", entry.time, reference, entry.event);
    }
    Ok(())
}

pub fn export(out: Option<PathBuf>) -> Result<()> {
    let source = timeline_path()?;
    let out = out.unwrap_or_else(|| PathBuf::from("timeline.jsonl"));
    fs::copy(&source, &out)?;
    println!("exported timeline to {}", out.display());
    Ok(())
}
