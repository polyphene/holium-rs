use crate::helpers::repo::setup_repo;
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use assert_fs::TempDir;
use std::path::{Path, PathBuf};

/***********************************************************
 * Constants useful to play around shaper testing
 ***********************************************************/

pub(crate) const SHAPER_NAME: &'static str = "shaper";
pub(crate) const SHAPER_ALTERNATIVE_NAME: &'static str = "alternative_shaper";

pub(crate) const JSON_SCHEMA: &'static str =
    r#"{ "type" : "array", "prefixItems" : [ {"type" : "string"} ] }"#;
pub(crate) const ALTERNATIVE_JSON_SCHEMA: &'static str =
    r#"{ "type" : "array", "prefixItems" : [ {"type" : "number"} ] }"#;
pub(crate) const NON_VALID_JSON_SCHEMA: &'static str = "{\"type\": \"wrong_type\"}";

/// Same as [setup_repo] but with a shaper already created
pub(crate) fn setup_repo_with_shaper() -> TempDir {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation
    let assert = build_shaper_create_cmd(repo_path, SHAPER_NAME, JSON_SCHEMA);
    // check output
    assert.success();

    repo
}

/// Create and run a create shaper command, returning an [Assert] used to validate testing
pub(crate) fn build_shaper_create_cmd(
    repo_path: &Path,
    shaper_name: &str,
    json_schema: &str,
) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("create")
        .arg(shaper_name)
        .arg("--json-schema")
        .arg(json_schema)
        .assert();
    assert
}

/// Create and run a delete shaper command, returning an [Assert] used to validate testing
pub(crate) fn build_shaper_delete_cmd(repo_path: &Path, shaper_name: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("delete")
        .arg(shaper_name)
        .assert();
    assert
}

/// Create and run a list shaper command, returning an [Assert] used to validate testing
pub(crate) fn build_shaper_list_cmd(repo_path: &Path) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("list")
        .assert();
    assert
}

/// Create and run a read shaper command, returning an [Assert] used to validate testing
pub(crate) fn build_shaper_read_cmd(repo_path: &Path, shaper_name: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("read")
        .arg(shaper_name)
        .assert();
    assert
}
