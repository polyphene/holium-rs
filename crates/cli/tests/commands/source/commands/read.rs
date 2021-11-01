use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::helpers::repo::setup_repo;
use crate::helpers::source::{build_source_read_cmd, setup_repo_with_source, SOURCE_NAME};

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("source").arg("read").arg("--help").assert();
    // Check success
    assert.success();
}


#[test]
fn cannot_read_source_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to read source without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("read")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn cannot_read_non_existent_source() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to read source
    let assert = build_source_read_cmd(repo_path, SOURCE_NAME);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("missing object for key"));
}

#[test]
fn can_read_source() {
    // initialize a repository
    let repo = setup_repo_with_source();
    let repo_path = repo.path();

    // try to read source
    let assert = build_source_read_cmd(repo_path, SOURCE_NAME);
    // check output
    assert
        .success()
        .stdout(predicate::str::contains(SOURCE_NAME))
        .stdout(predicate::str::contains("\"type\": \"string\""));
}
