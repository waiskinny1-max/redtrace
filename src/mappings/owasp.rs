use anyhow::Result;

pub fn add(finding_id: String, id: String) -> Result<()> {
    let mut finding = crate::findings::commands::load(&finding_id)?;
    if !finding.owasp_mappings.contains(&id) {
        finding.owasp_mappings.push(id);
        finding.updated_at = chrono::Utc::now();
        crate::findings::commands::save(&finding)?;
    }
    println!("updated OWASP mappings for {finding_id}");
    Ok(())
}
