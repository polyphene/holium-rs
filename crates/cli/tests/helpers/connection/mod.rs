use std::path::{Path, PathBuf};
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use assert_fs::TempDir;
use crate::helpers::repo::setup_repo;
use crate::helpers::shaper::SHAPER_ALTERNATIVE_NAME;
use crate::helpers::source::{
    build_source_create_cmd,
    JSON_SCHEMA as SOURCE_JSON_SCHEMA,
    SOURCE_ALTERNATIVE_NAME,
    SOURCE_NAME
};
use crate::helpers::transformation::{
    build_transformation_create_cmd,
    JSON_SCHEMA as TRANSFORMATION_JSON_SCHEMA,
    SOUND_BYTECODE,
    TRANSFORMATION_ALTERNATIVE_NAME,
    TRANSFORMATION_HANDLE,
    TRANSFORMATION_NAME
};

/***********************************************************
 * Constants useful to play around connection testing
 ***********************************************************/

pub(crate) const CONNECTION_LINKING_DELIMITER: &'static str = "→";
pub(crate) const NODE_TYPE_NAME_DELIMITER: &'static str = ":";

pub(crate) const SOURCE_TYPE: &'static str = "source";
pub(crate) const TRANSFORMATION_TYPE: &'static str = "transformation";
pub(crate) const SHAPER_TYPE: &'static str = "shaper";
pub(crate) const NON_VALID_TYPE: &'static str = "non_valid_type";

pub(crate) const SELECTOR: &'static str = "{ \".\": {} }";
pub(crate) const ALTERNATIVE_SELECTOR: &'static str = "{ \"i\": { \"i\": 1, \">\": { \".\": {} } } }";
pub(crate) const NON_VALID_SELECTOR: &'static str = "{ \"non\": \"valid\"}";

pub(crate) const NON_VALID_CONNECTION_ID: &'static str = "non_valid_connection_id";
pub(crate) fn default_connection_id() -> String {
    format!(
        "{}{}{}{}{}{}{}",
        SOURCE_TYPE,
        NODE_TYPE_NAME_DELIMITER,
        SOURCE_NAME,
        CONNECTION_LINKING_DELIMITER,
        TRANSFORMATION_TYPE,
        NODE_TYPE_NAME_DELIMITER,
        TRANSFORMATION_NAME
    )
}

pub(crate) fn node_type_name_alternative_pairs() -> Vec<(&'static str, &'static str)> {
    vec![
        (SOURCE_TYPE, SOURCE_ALTERNATIVE_NAME),
        (TRANSFORMATION_TYPE, TRANSFORMATION_ALTERNATIVE_NAME),
        (SHAPER_TYPE, SHAPER_ALTERNATIVE_NAME)
    ]
}

/// Same as [setup_repo] but with a source and a transformation already created
pub(crate) fn setup_repo_source_transformation() -> TempDir {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add source
    let assert = build_source_create_cmd(
        repo_path,
        SOURCE_NAME,
        SOURCE_JSON_SCHEMA,
    );
    // check output
    assert.success();

    // try to add transformation
    let assert = build_transformation_create_cmd(
        repo_path,
        TRANSFORMATION_NAME,
        TRANSFORMATION_HANDLE,
        SOUND_BYTECODE,
        TRANSFORMATION_JSON_SCHEMA,
        TRANSFORMATION_JSON_SCHEMA
    );
    // check output
    assert.success();

    repo
}


/// Same as [setup_repo_source_transformation] but also with a connection already created
pub(crate) fn setup_repo_connection() -> TempDir {
    // initialize a repository
    let repo = setup_repo_source_transformation();
    let repo_path = repo.path();

    // try to create connection
    let assert = build_connection_create_cmd(
        repo_path,
        SOURCE_TYPE,
        SOURCE_NAME,
        SELECTOR,
        TRANSFORMATION_TYPE,
        TRANSFORMATION_NAME,
        SELECTOR
    );
    // check output
    assert.success();

    repo
}

/// Create and run a create connection command, returning an [Assert] used to validate testing
pub(crate) fn build_connection_create_cmd(
    repo_path: &Path,
    tail_type: &str,
    tail_name: &str,
    tail_selector: &str,
    head_type: &str,
    head_name: &str,
    head_selector: &str
) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("create")
        .arg("--tail-type")
        .arg(tail_type)
        .arg("--tail-name")
        .arg(tail_name)
        .arg("--tail-selector")
        .arg(tail_selector)
        .arg("--head-type")
        .arg(head_type)
        .arg("--head-name")
        .arg(head_name)
        .arg("--head-selector")
        .arg(head_selector)
        .assert();
    assert
}

/// Create and run a delete connection command, returning an [Assert] used to validate testing
pub(crate) fn build_connection_delete_cmd(repo_path: &Path, connection_id: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("delete")
        .arg(connection_id)
        .assert();
    assert
}

/// Create and run a list connection command, returning an [Assert] used to validate testing
pub(crate) fn build_connection_list_cmd(repo_path: &Path) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("list")
        .assert();
    assert
}

/// Create and run a read connection command, returning an [Assert] used to validate testing
pub(crate) fn build_connection_read_cmd(repo_path: &Path, connection_id: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("read")
        .arg(connection_id)
        .assert();
    assert
}
