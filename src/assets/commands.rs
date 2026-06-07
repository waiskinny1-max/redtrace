use super::model::{Asset, AssetNote, AssetStore};
use crate::scope::guard::check_target;
use crate::scope::model::ScopeCheckStatus;
use crate::workspace::{next_sequential_id, read_yaml_or_default, require_workspace, write_yaml};
use anyhow::{bail, Result};
use chrono::Utc;

fn store_path() -> Result<std::path::PathBuf> {
    Ok(require_workspace()?.join("assets.yaml"))
}

pub fn load_store() -> Result<AssetStore> {
    read_yaml_or_default(&store_path()?)
}

fn save_store(store: &AssetStore) -> Result<()> {
    write_yaml(&store_path()?, store)
}

pub fn add(
    hostname: String,
    ip: Option<String>,
    asset_type: String,
    environment: Option<String>,
    force_out_of_scope: bool,
) -> Result<()> {
    let mut store = load_store()?;
    let scope_store = crate::scope::commands::load_store()?;
    let target = ip.clone().unwrap_or_else(|| hostname.clone());
    let scope_status = check_target(&scope_store, &target);

    if matches!(scope_status, ScopeCheckStatus::OutOfScope | ScopeCheckStatus::Excluded) && !force_out_of_scope {
        eprintln!("WARNING: asset does not match authorized scope or is explicitly excluded.");
        eprintln!("Use --force-out-of-scope if this is intentional and documented by the ROE.");
    }

    let now = Utc::now();
    let id = next_sequential_id("A", store.assets.iter().map(|asset| asset.id.clone()));
    let asset = Asset {
        id: id.clone(),
        hostname,
        ip,
        asset_type,
        environment,
        tags: vec![],
        scope_status,
        notes: vec![],
        created_at: now,
        updated_at: now,
    };

    store.assets.push(asset);
    save_store(&store)?;
    println!("added asset {id}");
    Ok(())
}

pub fn list() -> Result<()> {
    let store = load_store()?;
    for asset in store.assets {
        println!("{} {} {} {:?}", asset.id, asset.hostname, asset.scope_status, asset.ip);
    }
    Ok(())
}

pub fn show(id: String) -> Result<()> {
    let store = load_store()?;
    let Some(asset) = store.assets.iter().find(|asset| asset.id == id) else {
        bail!("asset not found: {id}");
    };
    println!("{} — {}", asset.id, asset.hostname);
    println!("type: {}", asset.asset_type);
    println!("ip: {}", asset.ip.clone().unwrap_or_else(|| "n/a".to_string()));
    println!("scope: {}", asset.scope_status);
    println!("tags: {}", asset.tags.join(", "));
    for note in &asset.notes {
        println!("note [{}]: {}", note.created_at, note.text);
    }
    Ok(())
}

pub fn tag(id: String, tag: String) -> Result<()> {
    let mut store = load_store()?;
    let Some(asset) = store.assets.iter_mut().find(|asset| asset.id == id) else {
        bail!("asset not found: {id}");
    };
    if !asset.tags.contains(&tag) {
        asset.tags.push(tag.clone());
    }
    asset.updated_at = Utc::now();
    save_store(&store)?;
    println!("tagged {id} with {tag}");
    Ok(())
}

pub fn note(id: String, text: String) -> Result<()> {
    let mut store = load_store()?;
    let Some(asset) = store.assets.iter_mut().find(|asset| asset.id == id) else {
        bail!("asset not found: {id}");
    };
    asset.notes.push(AssetNote { text, created_at: Utc::now() });
    asset.updated_at = Utc::now();
    save_store(&store)?;
    println!("added note to {id}");
    Ok(())
}
