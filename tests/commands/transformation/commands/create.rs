use crate::helpers::repo::setup_repo;
use crate::helpers::transformation::*;

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .arg("transformation")
        .arg("create")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_create_transformation_outside_repo() {
    // work in an empty directory
    let temp_dir = assert_fs::TempDir::new().unwrap();
    // try to create transformation
    let assert = build_transformation_create_cmd(
        temp_dir.path(),
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        JSON_SCHEMA,
        JSON_SCHEMA,
    );
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("inside a Holium repository"));
}

#[test]
fn cannot_create_transformation_without_any_positional_arg() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation without positional argument
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("--bytecode"));
}

#[test]
fn cannot_create_transformation_without_bytecode() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation without bytecode
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg(TRANSFORMATION_NAME)
        .arg("--handle")
        .arg(TRANSFORMATION_HANDLE)
        .arg("--json-schema-in")
        .arg(JSON_SCHEMA)
        .arg("--json-schema-out")
        .arg(JSON_SCHEMA)
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("--bytecode <FILE>"));
}

#[test]
fn cannot_create_transformation_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // get bytecode path
    let bytecode_path = bytecode_path(SOUND_BYTECODE);
    // try to create transformation without name
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg("--handle")
        .arg(TRANSFORMATION_HANDLE)
        .arg("--bytecode")
        .arg(bytecode_path)
        .arg("--json-schema-in")
        .arg(JSON_SCHEMA)
        .arg("--json-schema-out")
        .arg(JSON_SCHEMA)
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn cannot_create_transformation_without_handle() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // get bytecode path
    let bytecode_path = bytecode_path(SOUND_BYTECODE);
    // try to create transformation without handle
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg(TRANSFORMATION_NAME)
        .arg("--bytecode")
        .arg(bytecode_path)
        .arg("--json-schema-in")
        .arg(JSON_SCHEMA)
        .arg("--json-schema-out")
        .arg(JSON_SCHEMA)
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("--handle <HANDLE>"));
}

#[test]
fn cannot_create_transformation_without_json_schema_in() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // get bytecode path
    let bytecode_path = bytecode_path(SOUND_BYTECODE);
    // try to create transformation without json schema in
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg(TRANSFORMATION_NAME)
        .arg("--handle")
        .arg(TRANSFORMATION_HANDLE)
        .arg("--bytecode")
        .arg(bytecode_path)
        .arg("--json-schema-out")
        .arg(JSON_SCHEMA)
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains(
        "--json-schema-in <JSON-SCHEMA-IN>",
    ));
}

#[test]
fn cannot_create_transformation_without_json_schema_out() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // get bytecode path
    let bytecode_path = bytecode_path(SOUND_BYTECODE);
    // try to create transformation without json schema out
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg(TRANSFORMATION_NAME)
        .arg("--handle")
        .arg(TRANSFORMATION_HANDLE)
        .arg("--bytecode")
        .arg(bytecode_path)
        .arg("--json-schema-in")
        .arg(JSON_SCHEMA)
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains(
        "--json-schema-out <JSON-SCHEMA-OUT>",
    ));
}

#[test]
fn cannot_create_transformation_which_bytecode_lacks_wasm_magic_number() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation with corrupted wasm file
    let assert = build_transformation_create_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        CORRUPTED_BYTECODE,
        JSON_SCHEMA,
        JSON_SCHEMA,
    );
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid WebAssembly bytecode"));
}

#[test]
fn cannot_create_transformation_with_incorrect_json_schema_in() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation with incorrect json schema in
    let assert = build_transformation_create_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        NON_VALID_JSON_SCHEMA,
        JSON_SCHEMA,
    );
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("json schema"));
}

#[test]
fn cannot_create_transformation_with_incorrect_json_schema_out() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation with incorrect json schema out
    let assert = build_transformation_create_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        JSON_SCHEMA,
        NON_VALID_JSON_SCHEMA,
    );
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("json schema"));
}

#[test]
fn cannot_create_transformation_with_non_valid_json_object_in() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation with non valid json object in
    let assert = build_transformation_create_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        "",
        JSON_SCHEMA,
    );
    // check output
    assert.failure().stderr(predicate::str::contains(
        "invalid string can not be parsed to json",
    ));
}

#[test]
fn cannot_create_transformation_with_non_valid_json_object_out() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation with non valid json object out
    let assert = build_transformation_create_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        JSON_SCHEMA,
        "",
    );
    // check output
    assert.failure().stderr(predicate::str::contains(
        "invalid string can not be parsed to json",
    ));
}

#[test]
fn can_create_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation
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

    //Read to verify elements
    let assert = build_transformation_read_cmd(repo_path, TRANSFORMATION_NAME);

    assert
        .success()
        .stdout(predicate::str::contains(TRANSFORMATION_HANDLE))
        .stdout(predicate::str::contains("141 B"))
        .stdout(predicate::str::contains("\"type\": \"string\""));
}

#[test]
fn can_create_transformation_with_same_options_but_different_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create transformation
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

    // try to create same transformation with different name
    let assert = build_transformation_create_cmd(
        repo_path,
        TRANSFORMATION_ALTERNATIVE_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        JSON_SCHEMA,
        JSON_SCHEMA,
    );
    // check output
    assert.success();
}
