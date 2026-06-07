use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn validation_warns_about_incomplete_finding() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["init", "Demo"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["finding", "new", "Incomplete finding"]).assert().success();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Finding F-001 has no severity"));
}
