use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;

mod commands;

/************************
Test helper functions
 ************************/

/// Sets up a Holium repository in a temporary directory with no SCM and no DVC
fn setup_repo() -> TempDir {
    // initialize a repository
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--no-scm")
        .arg("--no-dvc")
        .assert();
    // check success message
    assert
        .success()
        .stdout(predicate::str::contains("Initialized Holium repository."));
    // return repository directory
    temp_dir
}
