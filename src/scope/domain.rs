pub fn domain_matches(value: &str, rule: &str) -> bool {
    let candidate = value.trim().trim_end_matches('.').to_ascii_lowercase();
    let rule = rule.trim().trim_end_matches('.').to_ascii_lowercase();

    if let Some(suffix) = rule.strip_prefix("*.") {
        candidate == suffix || candidate.ends_with(&format!(".{suffix}"))
    } else {
        candidate == rule
    }
}
