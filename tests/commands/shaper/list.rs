use std::path::{Path, PathBuf};

use crate::helpers::repo::setup_repo;
use crate::helpers::shaper::{build_shaper_read_cmd, setup_repo_with_shaper, SHAPER_NAME};
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd.arg("shaper").arg("list").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn can_list_with_no_shaper() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to list shaper
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("list")
        .assert();
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("no object in the list"));
}

#[test]
fn can_list_with_shaper() {
    // initialize a repository
    let repo = setup_repo_with_shaper();
    let repo_path = repo.path();

    // try to list shaper
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("list")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains(SHAPER_NAME));

    //Read to verify elements
    let assert = build_shaper_read_cmd(repo_path, SHAPER_NAME);

    assert
        .success()
        .stdout(predicate::str::contains(SHAPER_NAME))
        .stdout(predicate::str::contains("\"type\": \"string\""));
}
