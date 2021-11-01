use assert_cmd::Command;
use crate::helpers::connection::{build_connection_create_cmd, default_connection_id, NON_VALID_CONNECTION_ID, SELECTOR, setup_repo_connection, setup_repo_source_transformation, SOURCE_TYPE, TRANSFORMATION_TYPE};
use crate::helpers::repo::setup_repo;
use crate::helpers::source::{build_source_create_cmd, SOURCE_NAME, JSON_SCHEMA as SOURCE_JSON_SCHEMA};
use crate::helpers::transformation::{build_transformation_create_cmd, SOUND_BYTECODE, TRANSFORMATION_HANDLE, TRANSFORMATION_NAME, JSON_SCHEMA as TRANSFORMATION_JSON_SCHEMA};

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .arg("connection")
        .arg("delete")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_delete_connection_without_id() {
    // setup repo
    let repo = setup_repo_source_transformation();
    let repo_path = repo.path();

    // try to delete connection without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("delete")
        .assert();

    assert
        .failure()
        .stderr(predicates::str::contains("required arguments were not provided"))
        .stderr(predicates::str::contains("<ID>"));
}

#[test]
fn cannot_delete_non_existent_connection() {
    // setup repo
    let repo = setup_repo_source_transformation();
    let repo_path = repo.path();

    // try to delete connection without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("delete")
        .arg(NON_VALID_CONNECTION_ID)
        .assert();

    assert
        .failure()
        .stderr(predicates::str::contains("missing object for key"))
        .stderr(predicates::str::contains(NON_VALID_CONNECTION_ID));
}

#[test]
fn can_delete_connection() {
    // initialize a repository
    let repo = setup_repo_connection();
    let repo_path = repo.path();

    // try to delete connection without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("delete")
        .arg(default_connection_id().as_str())
        .assert();

    assert
        .success()
        .stdout(predicates::str::contains("object deleted"))
        .stdout(predicates::str::contains(default_connection_id().as_str()));
}