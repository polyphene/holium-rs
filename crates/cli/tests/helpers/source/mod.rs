use std::path::{Path, PathBuf};
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use assert_fs::TempDir;
use crate::helpers::repo::setup_repo;

/***********************************************************
 * Constants useful to play around source testing
 ***********************************************************/

pub(crate) const SOURCE_NAME: &'static str = "source";
pub(crate) const SOURCE_ALTERNATIVE_NAME: &'static str = "alternative_source";

pub(crate) const JSON_SCHEMA: &'static str = "{\"type\": \"string\"}";
pub(crate) const ALTERNATIVE_JSON_SCHEMA: &'static str = "{\"type\": \"number\"}";
pub(crate) const NON_VALID_JSON_SCHEMA: &'static str = "{\"type\": \"wrong_type\"}";

/// Same as [setup_repo] but with a source already created
pub(crate) fn setup_repo_with_source() -> TempDir {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation
    let assert = build_source_create_cmd(
        repo_path,
        SOURCE_NAME,
        JSON_SCHEMA,
    );
    // check output
    assert.success();

    repo
}

/// Create and run a create source command, returning an [Assert] used to validate testing
pub(crate) fn build_source_create_cmd(
    repo_path: &Path,
    source_name: &str,
    json_schema: &str,
) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("create")
        .arg(source_name)
        .arg("--json-schema")
        .arg(json_schema)
        .assert();
    assert
}

/// Create and run a delete source command, returning an [Assert] used to validate testing
pub(crate) fn build_source_delete_cmd(repo_path: &Path, source_name: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("delete")
        .arg(source_name)
        .assert();
    assert
}

/// Create and run a list source command, returning an [Assert] used to validate testing
pub(crate) fn build_source_list_cmd(repo_path: &Path) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("list")
        .assert();
    assert
}

/// Create and run a read source command, returning an [Assert] used to validate testing
pub(crate) fn build_source_read_cmd(repo_path: &Path, source_name: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("read")
        .arg(source_name)
        .assert();
    assert
}
