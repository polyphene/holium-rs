use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;
use crate::helpers::transformation::*;

/************************
Test helper functions
 ************************/

/// Sets up a Holium repository in a temporary directory with no SCM and no DVC
pub(crate) fn setup_repo() -> TempDir {
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

/// Same as [setup_repo] but with a transformation already created
pub(crate) fn setup_repo_with_transformation() -> TempDir {
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

    repo
}