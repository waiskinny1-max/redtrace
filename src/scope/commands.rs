use super::guard::{check_target, classify_rule};
use super::model::{ScopeExclusion, ScopeStatus, ScopeStore};
use crate::workspace::{next_sequential_id, read_yaml_or_default, require_workspace, write_yaml};
use anyhow::Result;
use chrono::Utc;

fn store_path() -> Result<std::path::PathBuf> {
    Ok(require_workspace()?.join("scope.yaml"))
}

pub fn load_store() -> Result<ScopeStore> {
    read_yaml_or_default(&store_path()?)
}

fn save_store(store: &ScopeStore) -> Result<()> {
    write_yaml(&store_path()?, store)
}

pub fn add(value: String, label: Option<String>, notes: Option<String>) -> Result<()> {
    let mut store = load_store()?;
    let id = next_sequential_id("SCOPE", store.rules.iter().map(|rule| rule.id.clone()));
    let rule = super::model::ScopeRule {
        id: id.clone(),
        rule_type: classify_rule(&value),
        value,
        label,
        status: ScopeStatus::InScope,
        notes,
        created_at: Utc::now(),
    };
    store.rules.push(rule);
    save_store(&store)?;
    println!("added scope rule {id}");
    Ok(())
}

pub fn exclude(value: String, reason: String) -> Result<()> {
    let mut store = load_store()?;
    let id = next_sequential_id("EXCL", store.exclusions.iter().map(|exclusion| exclusion.id.clone()));
    let exclusion = ScopeExclusion {
        id: id.clone(),
        rule_type: classify_rule(&value),
        value,
        reason,
        created_at: Utc::now(),
    };
    store.exclusions.push(exclusion);
    save_store(&store)?;
    println!("added exclusion {id}");
    Ok(())
}

pub fn list() -> Result<()> {
    let store = load_store()?;
    println!("Scope rules:");
    for rule in store.rules {
        println!("{} {:?} {} {}", rule.id, rule.rule_type, rule.value, rule.label.unwrap_or_default());
    }
    println!("Exclusions:");
    for exclusion in store.exclusions {
        println!("{} {:?} {} — {}", exclusion.id, exclusion.rule_type, exclusion.value, exclusion.reason);
    }
    Ok(())
}

pub fn check(target: String) -> Result<()> {
    let store = load_store()?;
    let status = check_target(&store, &target);
    match status {
        super::model::ScopeCheckStatus::InScope => println!("IN SCOPE\n{target} matches an authorized scope rule."),
        super::model::ScopeCheckStatus::Excluded => println!("EXCLUDED\n{target} matches an explicit exclusion."),
        super::model::ScopeCheckStatus::OutOfScope => println!("OUT OF SCOPE\nNo matching scope rule found for {target}."),
        super::model::ScopeCheckStatus::Unknown => println!("UNKNOWN\nCould not determine scope status for {target}."),
    }
    Ok(())
}
