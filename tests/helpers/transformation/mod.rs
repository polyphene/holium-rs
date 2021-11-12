use crate::helpers::repo::setup_repo;
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use assert_fs::TempDir;
use std::path::{Path, PathBuf};

/***********************************************************
 * Constants useful to play around transformation testing
 ***********************************************************/

pub(crate) const TRANSFORMATION_NAME: &'static str = "import_transformation";
pub(crate) const TRANSFORMATION_ALTERNATIVE_NAME: &'static str = "alternative_transformation";

pub(crate) const TRANSFORMATION_HANDLE: &'static str = "helloWorld";
pub(crate) const TRANSFORMATION_ALTERNATIVE_HANDLE: &'static str = "alternative_transformation";

pub(crate) const SOUND_BYTECODE: &'static str = "import.wasm";
pub(crate) const ALTERNATIVE_BYTECODE: &'static str = "alternative_import.wasm";
pub(crate) const CORRUPTED_BYTECODE: &'static str = "import_corrupted.wasm";

pub(crate) const JSON_SCHEMA: &'static str =
    r#"{ "type" : "array", "prefixItems" : [ {"type" : "string"} ] }"#;
pub(crate) const ALTERNATIVE_JSON_SCHEMA: &'static str =
    r#"{ "type" : "array", "prefixItems" : [ {"type" : "number"} ] }"#;
pub(crate) const NON_VALID_JSON_SCHEMA: &'static str = "{\"type\": \"string\"}";

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

/// Returns a full path to the wasm bytecode in the test assets for transformations
pub(crate) fn bytecode_path(transformation_filename: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("helpers")
        .join("transformation")
        .join("assets")
        .join(transformation_filename)
}

/// Create and run a create transformation command, returning an [Assert] used to validate testing
pub(crate) fn build_transformation_create_cmd(
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

/// Create and run a delete transformation command, returning an [Assert] used to validate testing
pub(crate) fn build_transformation_delete_cmd(
    repo_path: &Path,
    transformation_name: &str,
) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("delete")
        .arg(transformation_name)
        .assert();
    assert
}

/// Create and run a list transformation command, returning an [Assert] used to validate testing
pub(crate) fn build_transformation_list_cmd(repo_path: &Path) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("list")
        .assert();
    assert
}

/// Create and run a read transformation command, returning an [Assert] used to validate testing
pub(crate) fn build_transformation_read_cmd(repo_path: &Path, transformation_name: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("read")
        .arg(transformation_name)
        .assert();
    assert
}
