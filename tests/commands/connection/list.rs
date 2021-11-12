use crate::helpers::connection::{
    build_connection_read_cmd, default_connection_id, setup_repo_with_connection,
};
use crate::helpers::repo::setup_repo;

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd.arg("connection").arg("list").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn can_list_with_no_connection() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to list connection
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("list")
        .assert();
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("no object in the list"));
}

#[test]
fn can_list_with_connection() {
    // initialize a repository
    let repo = setup_repo_with_connection();
    let repo_path = repo.path();

    // try to list connection
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("list")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains(default_connection_id().as_str()));

    //Read to verify elements
    let assert = build_connection_read_cmd(repo_path, default_connection_id().as_str());

    assert
        .success()
        .stdout(predicate::str::contains(default_connection_id().as_str()))
        .stdout(predicate::str::contains("\".\": {}"));
}
