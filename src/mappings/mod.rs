pub mod attack;
pub mod csf;
pub mod owasp;

use anyhow::Result;

pub fn list(finding_id: String) -> Result<()> {
    let finding = crate::findings::commands::load(&finding_id)?;
    println!("Mappings for {}", finding.id);
    for mapping in finding.attack_mappings {
        println!("ATT&CK: {} / {}", mapping.tactic, mapping.technique);
    }
    for mapping in finding.owasp_mappings {
        println!("OWASP WSTG: {mapping}");
    }
    for mapping in finding.csf_mappings {
        println!("NIST CSF: {mapping}");
    }
    Ok(())
}
