use std::path::{Path, PathBuf};

use crate::setup_repo;
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;

const TRANSFORMATION_NAME: &'static str = "import_transformation";
const TRANSFORMATION_ALTERNATIVE_NAME: &'static str = "alternative_transformation";
const TRANSFORMATION_HANDLE: &'static str = "helloWorld";

const SOUND_BYTECODE: &'static str = "import.wasm";
const CORRUPTED_BYTECODE: &'static str = "import_corrupted.wasm";

fn bytecode_path(transformation_filename: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("commands")
        .join("transformation")
        .join("assets")
        .join(transformation_filename)
}

fn build_transformation_add_cmd(
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
    // try to add transformation
    let assert = build_transformation_add_cmd(
        temp_dir.path(),
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        "{\"type\": \"string\"}",
        "{\"type\": \"string\"}",
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
    // try to add transformation without positional argument
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("bytecode"));
}

#[test]
fn cannot_create_transformation_without_bytecode() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation without bytecode
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg(TRANSFORMATION_NAME)
        .arg("--handle")
        .arg(TRANSFORMATION_HANDLE)
        .arg("--json-schema-in")
        .arg("{\"type\": \"string\"}")
        .arg("--json-schema-out")
        .arg("{\"type\": \"string\"}")
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
    // try to add transformation without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg("--handle")
        .arg(TRANSFORMATION_HANDLE)
        .arg("--bytecode")
        .arg(bytecode_path)
        .arg("--json-schema-in")
        .arg("{\"type\": \"string\"}")
        .arg("--json-schema-out")
        .arg("{\"type\": \"string\"}")
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
    // try to add transformation without handle
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("create")
        .arg(TRANSFORMATION_NAME)
        .arg("--bytecode")
        .arg(bytecode_path)
        .arg("--json-schema-in")
        .arg("{\"type\": \"string\"}")
        .arg("--json-schema-out")
        .arg("{\"type\": \"string\"}")
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
    // try to add transformation without json schema in
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
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
        .arg("{\"type\": \"string\"}")
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
    // try to add transformation without json schema out
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
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
        .arg("{\"type\": \"string\"}")
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
    // try to add transformation with corrupted wasm file
    let assert = build_transformation_add_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        CORRUPTED_BYTECODE,
        "{\"type\": \"string\"}",
        "{\"type\": \"string\"}",
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
    // try to add transformation with invalid json schema in
    let assert = build_transformation_add_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        "{\"type\": \"integer\"}",
        "{\"type\": \"string\"}",
    );
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid json schema"));
}

#[test]
fn cannot_create_transformation_with_incorrect_json_schema_out() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation with invalid json schema out
    let assert = build_transformation_add_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        "{\"type\": \"string\"}",
        "{\"type\": \"integer\"}",
    );
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid json schema"));
}

#[test]
fn can_create_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation
    let assert = build_transformation_add_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        "{\"type\": \"string\"}",
        "{\"type\": \"string\"}",
    );
    // check output
    assert.success();
}

#[test]
fn can_create_transformation_with_same_options_but_different_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation
    let assert = build_transformation_add_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        "{\"type\": \"string\"}",
        "{\"type\": \"string\"}",
    );
    // check output
    assert.success();

    // try to add same transformation with different name
    let assert = build_transformation_add_cmd(
        repo_path,
        TRANSFORMATION_ALTERNATIVE_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        "{\"type\": \"string\"}",
        "{\"type\": \"string\"}",
    );
    // check output
    assert.success();
}
