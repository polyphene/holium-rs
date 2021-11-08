use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::helpers::repo::setup_repo;
use crate::helpers::shaper::{build_shaper_delete_cmd, build_shaper_list_cmd, setup_repo_with_shaper, SHAPER_NAME};

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .arg("shaper")
        .arg("delete")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_delete_shaper_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to delete shaper without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("delete")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn cannot_delete_non_existent_shaper() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to delete shaper
    let assert = build_shaper_delete_cmd(repo_path, SHAPER_NAME);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("missing object for key"));
}

#[test]
fn can_delete_shaper() {
    // initialize a repository
    let repo = setup_repo_with_shaper();
    let repo_path = repo.path();

    // try to delete shaper
    let assert = build_shaper_delete_cmd(repo_path, SHAPER_NAME);
    // check output
    assert.success();

    //list to check delete worked
    let assert = build_shaper_list_cmd(repo_path);
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("no object in the list"));
}
