use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn markdown_report_contains_finding_and_evidence() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["init", "Demo", "--roe", "Lab only"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["scope", "add", "10.10.0.0/24"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["asset", "add", "web01.lab.local", "--ip", "10.10.0.20", "--type", "web"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["finding", "new", "Weak access control"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["finding", "set", "F-001", "--severity", "high", "--asset", "A-001", "--recommendation", "Fix authorization", "--summary", "Missing check", "--impact", "Unauthorized access", "--confidence", "confirmed"]).assert().success();
    fs::write(dir.path().join("evidence.txt"), "demo evidence").unwrap();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["evidence", "add", "evidence.txt", "--finding", "F-001"]).assert().success();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["report", "--format", "markdown", "--out", "report.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("generated Markdown full report"));

    let report = fs::read_to_string(dir.path().join("report.md")).unwrap();
    assert!(report.contains("Weak access control"));
    assert!(report.contains("EV-001"));
    assert!(report.contains("Findings Severity Summary"));
}

#[test]
fn executive_report_omits_evidence_hash_index() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["init", "Demo", "--roe", "Lab only"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["finding", "new", "Weak access control"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["finding", "set", "F-001", "--severity", "high", "--recommendation", "Fix authorization", "--summary", "Missing check", "--impact", "Unauthorized access"]).assert().success();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["report", "--format", "markdown", "--profile", "executive", "--out", "executive.md"])
        .assert()
        .success();

    let report = fs::read_to_string(dir.path().join("executive.md")).unwrap();
    assert!(report.contains("Report profile:** executive"));
    assert!(!report.contains("## 8. Evidence Index"));
}
