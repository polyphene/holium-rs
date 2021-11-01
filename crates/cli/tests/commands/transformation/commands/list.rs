use std::path::{Path, PathBuf};

use crate::helpers::repo::setup_repo;
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::helpers::transformation::*;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("list").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn can_list_with_no_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to list transformation
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("list")
        .assert();
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("no object in the list"));
}

#[test]
fn can_list_with_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation
    let assert = build_transformation_create_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        JSON_SCHEMA,
        JSON_SCHEMA,
    );
    // check output
    assert.success();

    // try to list transformation
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("list")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains(TRANSFORMATION_NAME));

    //Read to verify elements
    let assert = build_transformation_read_cmd(repo_path, TRANSFORMATION_NAME);

    assert
        .success()
        .stdout(predicate::str::contains(TRANSFORMATION_HANDLE))
        .stdout(predicate::str::contains("141 B"))
        .stdout(predicate::str::contains("\"type\": \"string\""));
}
