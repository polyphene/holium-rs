use crate::helpers::repo::setup_repo;
use crate::helpers::transformation::*;

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd.arg("transformation").arg("read").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_read_transformation_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to read transformation without name
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("read")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn cannot_read_non_existent_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to read non existent transformation
    let assert = build_transformation_read_cmd(repo_path, TRANSFORMATION_NAME);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("missing object for key"));
}

#[test]
fn can_read_transformation() {
    // initialize a repository
    let repo = setup_repo_with_transformation();
    let repo_path = repo.path();

    // try to read transformation
    let assert = build_transformation_read_cmd(repo_path, TRANSFORMATION_NAME);
    // check output
    assert.success();
}
