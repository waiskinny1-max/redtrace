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

#[test]
fn evidence_chain_can_be_exported() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["init", "Demo"]).assert().success();
    fs::write(dir.path().join("terminal.txt"), "demo evidence").unwrap();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["evidence", "add", "terminal.txt", "--type", "terminal-output"])
        .assert()
        .success();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["evidence", "chain", "--out", "chain-of-custody.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Evidence Chain"));

    let chain = fs::read_to_string(dir.path().join("chain-of-custody.md")).unwrap();
    assert!(chain.contains("Evidence Chain of Custody"));
    assert!(chain.contains("EV-001"));
    assert!(chain.contains("OK"));
}
