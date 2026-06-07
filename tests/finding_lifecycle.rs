use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn finding_can_be_created_and_updated() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["init", "Demo"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["finding", "new", "Weak access control"]).assert().success();
    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args([
            "finding",
            "set",
            "F-001",
            "--severity",
            "high",
            "--summary",
            "Server-side authorization was missing.",
            "--impact",
            "Privilege boundaries may be bypassed.",
            "--recommendation",
            "Enforce server-side authorization checks.",
            "--confidence",
            "confirmed",
        ])
        .assert()
        .success();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["finding", "show", "F-001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Weak access control"));
}
