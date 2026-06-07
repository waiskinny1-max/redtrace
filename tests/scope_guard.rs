use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn scope_check_detects_in_and_out_of_scope_targets() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["init", "Demo"]).assert().success();
    Command::cargo_bin("redtrace").unwrap().current_dir(dir.path()).args(["scope", "add", "10.10.0.0/24", "--label", "lab"]).assert().success();

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["scope", "check", "10.10.0.42"])
        .assert()
        .success()
        .stdout(predicate::str::contains("IN SCOPE"));

    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["scope", "check", "192.168.1.10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OUT OF SCOPE"));
}
