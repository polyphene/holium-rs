use std::path::{Path, PathBuf};

use crate::helpers::repo::setup_repo;
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;

const TRANSFORMATION_NAME: &'static str = "import_transformation";
const TRANSFORMATION_HANDLE: &'static str = "helloWorld";
const TRANSFORMATION_ALTERNATIVE_HANDLE: &'static str = "alternative_transformation";

const SOUND_BYTECODE: &'static str = "import.wasm";
const CORRUPTED_BYTECODE: &'static str = "import_corrupted.wasm";

const JSON_SCHEMA: &'static str = "{\"type\": \"string\"}";
const ALTERNATIVE_JSON_SCHEMA: &'static str = "{\"type\": \"number\"}";

fn bytecode_path(transformation_filename: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("commands")
        .join("transformation")
        .join("assets")
        .join(transformation_filename)
}

fn build_transformation_create_cmd(
    repo_path: &Path,
    transformation_name: &str,
    transformation_handle: &str,
    transformation_filename: &str,
    json_schema_in: &str,
    json_schema_out: &str,
) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let bytecode_path = bytecode_path(transformation_filename);
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg(transformation_name)
        .arg("--handle")
        .arg(transformation_handle)
        .arg("--bytecode")
        .arg(bytecode_path)
        .arg("--json-schema-in")
        .arg(json_schema_in)
        .arg("--json-schema-out")
        .arg(json_schema_out)
        .assert();
    assert
}

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
    // try to update transformation without positional argument
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
    // try to update transformation with invalid json schema out
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
    // try to update transformation with invalid json schema in
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .arg("--json-schema-in")
        .arg("{\"type\": \"wrong_type\"}")
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid json schema"));
}

#[test]
fn cannot_update_transformation_with_incorrect_json_schema_out() {
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
    // try to update transformation with invalid json schema out
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("update")
        .arg(TRANSFORMATION_NAME)
        .arg("--json-schema-out")
        .arg("{\"type\": \"wrong_type\"}")
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid json schema"));
}


#[test]
fn can_update_transformation() {
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
    // try to update transformation without positional argument
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
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("read")
        .arg(TRANSFORMATION_NAME)
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains(TRANSFORMATION_ALTERNATIVE_HANDLE))
        .stdout(predicate::str::contains("476 B"))
        .stdout(predicate::str::contains("\"type\": \"number\""));
}
