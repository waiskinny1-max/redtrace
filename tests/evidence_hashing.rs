use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn evidence_is_copied_hashed_and_verified() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["init", "Demo"]).assert().success();
    fs::write(dir.path().join("evidence.txt"), "demo evidence").unwrap();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["evidence", "add", "evidence.txt", "--type", "terminal-output"])
        .assert()
        .success();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["evidence", "verify", "EV-001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("EV-001 OK"));
}
