use std::path::{Path, PathBuf};

use crate::helpers::repo::setup_repo;
use crate::helpers::source::{build_source_read_cmd, setup_repo_with_source, SOURCE_NAME};
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("source").arg("list").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn can_list_with_no_source() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to list source
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("list")
        .assert();
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("no object in the list"));
}

#[test]
fn can_list_with_source() {
    // initialize a repository
    let repo = setup_repo_with_source();
    let repo_path = repo.path();

    // try to list source
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("list")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains(SOURCE_NAME));

    //Read to verify elements
    let assert = build_source_read_cmd(repo_path, SOURCE_NAME);

    assert
        .success()
        .stdout(predicate::str::contains(SOURCE_NAME))
        .stdout(predicate::str::contains("\"type\": \"string\""));
}
