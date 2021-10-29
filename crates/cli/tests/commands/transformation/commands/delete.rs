use std::path::{Path, PathBuf};

use crate::setup_repo;
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;

const TRANSFORMATION_NAME: &'static str = "import_transformation";
const TRANSFORMATION_ALTERNATIVE_NAME: &'static str = "alternative_transformation";
const TRANSFORMATION_HANDLE: &'static str = "helloWorld";

const SOUND_BYTECODE: &'static str = "import.wasm";

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

fn build_transformation_delete_cmd(repo_path: &Path, transformation_name: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("delete")
        .arg(transformation_name)
        .assert();
    assert
}

fn build_transformation_list_cmd(repo_path: &Path) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("list")
        .assert();
    assert
}

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .arg("transformation")
        .arg("delete")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_delete_non_existent_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to delete transformation
    let assert = build_transformation_delete_cmd(repo_path, TRANSFORMATION_NAME);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("missing object for key"));
}

#[test]
fn can_delete_transformation() {
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

    // try to delete transformation
    let assert = build_transformation_delete_cmd(repo_path, TRANSFORMATION_NAME);
    // check output
    assert.success();

    //list to check delete worked
    let assert = build_transformation_list_cmd(repo_path);
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("no object in the list"));
}
