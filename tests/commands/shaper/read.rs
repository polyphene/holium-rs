use crate::helpers::repo::setup_repo;
use crate::helpers::shaper::{build_shaper_read_cmd, setup_repo_with_shaper, SHAPER_NAME};
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd.arg("shaper").arg("read").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_read_shaper_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to read shaper without name
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("read")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn cannot_read_non_existent_shaper() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to read shaper
    let assert = build_shaper_read_cmd(repo_path, SHAPER_NAME);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("missing object for key"));
}

#[test]
fn can_read_shaper() {
    // initialize a repository
    let repo = setup_repo_with_shaper();
    let repo_path = repo.path();

    // try to read shaper
    let assert = build_shaper_read_cmd(repo_path, SHAPER_NAME);
    // check output
    assert
        .success()
        .stdout(predicate::str::contains(SHAPER_NAME))
        .stdout(predicate::str::contains("\"type\": \"string\""));
}
