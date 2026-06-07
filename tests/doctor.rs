use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn doctor_reports_missing_workspace_without_failing() {
    let dir = tempdir().unwrap();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["doctor"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Workspace:    MISSING"));
}
