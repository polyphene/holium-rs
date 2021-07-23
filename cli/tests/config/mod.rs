use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Output;
use assert_fs::TempDir;

/// Sets up a Holium repository in a temporary directory with no SCM and no DVC
fn setup_repo() -> TempDir {
    // initialize a repository
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--no-scm")
        .arg("--no-dvc")
        .assert();
    // check success message
    assert
        .success()
        .stdout(predicate::str::contains("Initialized Holium repository."));
    // return repository directory
    temp_dir
}

/// Parses a boolean from a command output.
/// Returns boolean if found, default otherwise (that can be None).
fn parse_bool_output(output: &Output, default_on_empty: Option<bool>) -> Option<bool> {
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    match stdout_str.trim() {
        "True\n" => Some(true),
        "False\n" => Some(false),
        "\n" => default_on_empty,
        _ => None
    }
}

#[test]
fn can_get_set_config() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to get initial configuration value
    let config_key = "core.no_scm";
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("config")
        .arg(config_key)
        .assert();
    // check success
    assert = assert.success();
    // check and parse output
    let initial_value = parse_bool_output(assert.get_output(), Some(false))
        .expect("invalid initial configuration bool value");
    // try to set config opposite value
    cmd = Command::cargo_bin("holium-cli").unwrap();
    assert = cmd
        .current_dir(repo_path)
        .arg("config")
        .arg(config_key)
        .arg((!initial_value).to_string())
        .assert();
    // check success
    assert.success();
    // try to get and check new config value
    cmd = Command::cargo_bin("holium-cli").unwrap();
    assert = cmd
        .current_dir(repo_path)
        .arg("config")
        .arg(config_key)
        .assert();
    // check success
    assert = assert.success();
    // parse output
    let new_value = parse_bool_output(assert.get_output(), Some(false))
        .expect("invalid configuration bool value");
    // check that the configuration has been changed
    assert_ne!(initial_value, new_value)
}