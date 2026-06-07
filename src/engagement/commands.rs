use super::{storage, Engagement};
use crate::error::RedtraceError;
use crate::workspace::{workspace_path, write_yaml, WORKSPACE_DIR};
use anyhow::{Context, Result};
use std::fs;

pub fn init(name: String, client: Option<String>, roe: Option<String>) -> Result<()> {
    let root = workspace_path();
    if root.exists() {
        return Err(RedtraceError::WorkspaceExists(root.display().to_string()).into());
    }

    fs::create_dir_all(root.join("findings"))?;
    fs::create_dir_all(root.join("evidence"))?;
    fs::create_dir_all(root.join("reports"))?;
    fs::create_dir_all(root.join("mappings"))?;

    let engagement = Engagement::new(name, client, roe);
    write_yaml(&root.join("engagement.yaml"), &engagement)?;
    write_yaml(&root.join("assets.yaml"), &crate::assets::model::AssetStore::default())?;
    write_yaml(&root.join("scope.yaml"), &crate::scope::model::ScopeStore::default())?;
    fs::write(root.join("timeline.jsonl"), b"").context("failed to initialize timeline")?;

    println!("initialized redtrace workspace at {WORKSPACE_DIR}/");
    println!("engagement: {}", engagement.name);
    Ok(())
}

pub fn status() -> Result<()> {
    let engagement = storage::load_engagement()?;
    let assets = crate::assets::commands::load_store()?;
    let scope = crate::scope::commands::load_store()?;
    let findings = crate::findings::commands::load_all()?;
    let evidence = crate::evidence::commands::load_all()?;
    let timeline_count = crate::timeline::commands::load_all()?.len();

    println!("redtrace status");
    println!("engagement: {}", engagement.name);
    if let Some(client) = engagement.client {
        println!("client: {client}");
    }
    println!("scope rules: {}", scope.rules.len());
    println!("scope exclusions: {}", scope.exclusions.len());
    println!("assets: {}", assets.assets.len());
    println!("findings: {}", findings.len());
    println!("evidence files: {}", evidence.len());
    println!("timeline events: {timeline_count}");
    Ok(())
}
