use crate::findings::model::AttackMapping;
use anyhow::Result;

pub fn add(finding_id: String, tactic: String, technique: String) -> Result<()> {
    let mut finding = crate::findings::commands::load(&finding_id)?;
    let exists = finding
        .attack_mappings
        .iter()
        .any(|mapping| mapping.tactic == tactic && mapping.technique == technique);
    if !exists {
        finding.attack_mappings.push(AttackMapping { tactic, technique });
        finding.updated_at = chrono::Utc::now();
        crate::findings::commands::save(&finding)?;
    }
    println!("updated ATT&CK mappings for {finding_id}");
    Ok(())
}
