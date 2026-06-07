use anyhow::Result;

pub fn add(finding_id: String, function: String) -> Result<()> {
    let normalized = function.to_ascii_lowercase();
    let mut finding = crate::findings::commands::load(&finding_id)?;
    if !finding.csf_mappings.contains(&normalized) {
        finding.csf_mappings.push(normalized);
        finding.updated_at = chrono::Utc::now();
        crate::findings::commands::save(&finding)?;
    }
    println!("updated NIST CSF mappings for {finding_id}");
    Ok(())
}
