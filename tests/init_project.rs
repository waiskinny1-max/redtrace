use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn init_creates_workspace() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("redtrace")
        .unwrap()
        .current_dir(dir.path())
        .args(["init", "Demo Engagement"])
        .assert()
        .success()
        .stdout(predicate::str::contains("initialized redtrace workspace"));

    assert!(dir.path().join(".redtrace/engagement.yaml").exists());
    assert!(dir.path().join(".redtrace/findings").is_dir());
    assert!(dir.path().join(".redtrace/evidence").is_dir());
}
