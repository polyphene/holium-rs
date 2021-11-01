use std::path::{Path, PathBuf};

use crate::helpers::repo::setup_repo;
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::helpers::transformation::*;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .arg("transformation")
        .arg("delete")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_delete_transformation_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to delete transformation without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("delete")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn cannot_delete_non_existent_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to delete transformation
    let assert = build_transformation_delete_cmd(repo_path, TRANSFORMATION_NAME);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("missing object for key"));
}

#[test]
fn can_delete_transformation() {
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

    // try to delete transformation
    let assert = build_transformation_delete_cmd(repo_path, TRANSFORMATION_NAME);
    // check output
    assert.success();

    //list to check delete worked
    let assert = build_transformation_list_cmd(repo_path);
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("no object in the list"));
}
