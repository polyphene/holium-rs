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
        .arg("update")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_update_transformation_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to update transformation without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn can_update_transformation_without_any_positional_arg() {
    // initialize a repository
    let repo = setup_repo_with_transformation();
    let repo_path = repo.path();

    // try to update transformation without positional argument
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .assert();
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("object updated"));
}

#[test]
fn cannot_update_transformation_which_bytecode_lacks_wasm_magic_number() {
    // initialize a repository
    let repo = setup_repo_with_transformation();
    let repo_path = repo.path();

    // try to update transformation with corrupted wasm bytecode
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .arg("--bytecode")
        .arg(bytecode_path(CORRUPTED_BYTECODE).to_str().unwrap())
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid WebAssembly bytecode"));
}

#[test]
fn cannot_update_transformation_with_incorrect_json_schema_in() {
    // initialize a repository
    let repo = setup_repo_with_transformation();
    let repo_path = repo.path();

    // try to update transformation with incorrect json schema in
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .arg("--json-schema-in")
        .arg(NON_VALID_JSON_SCHEMA)
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid json schema"));
}

#[test]
fn cannot_update_transformation_with_incorrect_json_schema_out() {
    // initialize a repository
    let repo = setup_repo_with_transformation();
    let repo_path = repo.path();

    // try to update transformation with incorrect json schema out
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .arg("--json-schema-out")
        .arg(NON_VALID_JSON_SCHEMA)
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid json schema"));
}


#[test]
fn cannot_update_transformation_with_non_valid_json_object_in() {
    // initialize a repository
    let repo = setup_repo_with_transformation();
    let repo_path = repo.path();

    // try to update transformation with non valid json object in
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .arg("--json-schema-in")
        .arg("")
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid string can not be parsed to json"));
}

#[test]
fn cannot_update_transformation_with_non_valid_json_object_out() {
    // initialize a repository
    let repo = setup_repo_with_transformation();
    let repo_path = repo.path();

    // try to update transformation with non valid json object out
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .arg("--json-schema-out")
        .arg("")
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid string can not be parsed to json"));
}


#[test]
fn can_update_transformation() {
    // initialize a repository
    let repo = setup_repo_with_transformation();
    let repo_path = repo.path();

    // try to update transformation
    let alternative_bytecode_path = bytecode_path("alternative_import.wasm");
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .arg("--bytecode")
        .arg(alternative_bytecode_path)
        .arg("--handle")
        .arg(TRANSFORMATION_ALTERNATIVE_HANDLE)
        .arg("--json-schema-in")
        .arg(ALTERNATIVE_JSON_SCHEMA)
        .arg("--json-schema-out")
        .arg(ALTERNATIVE_JSON_SCHEMA)
        .assert();
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("object updated"));

    //Read to verify elements
    let assert = build_transformation_read_cmd(repo_path, TRANSFORMATION_NAME);

    assert
        .success()
        .stdout(predicate::str::contains(TRANSFORMATION_ALTERNATIVE_HANDLE))
        .stdout(predicate::str::contains("476 B"))
        .stdout(predicate::str::contains("\"type\": \"number\""));
}
