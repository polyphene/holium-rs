use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::helpers::repo::setup_repo;
use crate::helpers::source::{build_source_delete_cmd, build_source_list_cmd, setup_repo_with_source, SOURCE_NAME};

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .arg("source")
        .arg("delete")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_delete_source_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to delete source without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("delete")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn cannot_delete_non_existent_source() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to delete source
    let assert = build_source_delete_cmd(repo_path, SOURCE_NAME);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("missing object for key"));
}

#[test]
fn can_delete_source() {
    // initialize a repository
    let repo = setup_repo_with_source();
    let repo_path = repo.path();

    // try to delete source
    let assert = build_source_delete_cmd(repo_path, SOURCE_NAME);
    // check output
    assert.success();

    //list to check delete worked
    let assert = build_source_list_cmd(repo_path);
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("no object in the list"));
}
