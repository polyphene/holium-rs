use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::helpers::repo::setup_repo;
use crate::helpers::connection::{build_connection_read_cmd, setup_repo_connection, default_connection_id};

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("connection").arg("read").arg("--help").assert();
    // Check success
    assert.success();
}


#[test]
fn cannot_read_connection_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to read connection without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("read")
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("<ID>"));
}

#[test]
fn cannot_read_non_existent_connection() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to read connection
    let assert = build_connection_read_cmd(repo_path, default_connection_id().as_str());
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("missing object for key"));
}

#[test]
fn can_read_connection() {
    // initialize a repository
    let repo = setup_repo_connection();
    let repo_path = repo.path();

    // try to read connection
    let assert = build_connection_read_cmd(repo_path, default_connection_id().as_str());
    // check output
    assert
        .success()
        .stdout(predicate::str::contains(default_connection_id().as_str()))
        .stdout(predicate::str::contains("\".\": {}"));
}
