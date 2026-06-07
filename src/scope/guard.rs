use super::cidr::ip_in_cidr;
use super::domain::domain_matches;
use super::model::{ScopeCheckStatus, ScopeRuleType, ScopeStore};
use std::net::IpAddr;

pub fn classify_rule(value: &str) -> ScopeRuleType {
    if value.contains('/') {
        ScopeRuleType::Cidr
    } else if value.starts_with("*.") {
        ScopeRuleType::WildcardDomain
    } else if value.parse::<IpAddr>().is_ok() {
        ScopeRuleType::Ip
    } else {
        ScopeRuleType::Domain
    }
}

pub fn check_target(store: &ScopeStore, target: &str) -> ScopeCheckStatus {
    for exclusion in &store.exclusions {
        if rule_matches(&exclusion.rule_type, &exclusion.value, target) {
            return ScopeCheckStatus::Excluded;
        }
    }

    for rule in &store.rules {
        if rule_matches(&rule.rule_type, &rule.value, target) {
            return ScopeCheckStatus::InScope;
        }
    }

    ScopeCheckStatus::OutOfScope
}

fn rule_matches(rule_type: &ScopeRuleType, rule_value: &str, target: &str) -> bool {
    match rule_type {
        ScopeRuleType::Cidr => ip_in_cidr(target, rule_value),
        ScopeRuleType::Ip => target.parse::<IpAddr>().is_ok() && target == rule_value,
        ScopeRuleType::Domain | ScopeRuleType::WildcardDomain => domain_matches(target, rule_value),
    }
}
