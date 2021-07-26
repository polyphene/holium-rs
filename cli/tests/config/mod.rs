use std::process::Output;

use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;

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
    // Get string from stdout
    let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
    // Try to match
    match stdout_str.trim() {
        "true" => Some(true),
        "false" => Some(false),
        "" => default_on_empty,
        _ => None
    }
}

#[test]
fn help_is_available_for_config_cmd() {
    // try to get help for the config command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("config").arg("--help").assert();
    // check success
    assert.success();
}

#[test]
fn can_get_empty_config_without_repo_config_file() {
    // work in an empty directory
    let temp_dir = assert_fs::TempDir::new().unwrap();
    // try to get config
    let config_key = "core.no_scm";
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(temp_dir.path())
        .arg("config")
        .arg(config_key)
        .assert();
    // check output
    assert.success().stdout(predicate::str::is_empty());
}

#[test]
fn a() {
    assert!("v = true"
        .parse::<toml::Value>()
        .unwrap()
        .as_table()
        .unwrap()
        .get("v")
        .unwrap()
        .as_bool()
        .unwrap());
    // assert!("[true]".parse::<toml::Value>().unwrap().as_array().unwrap()[0].as_bool().unwrap());
    assert!("true".parse::<toml::Value>().is_err());

    use serde_derive::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    struct S {
        k: toml::Value,
    }
    let s: S = toml::from_str(
        r#"
        k = true
    "#,
    )
    .unwrap();
    assert!(s.k.as_bool().unwrap())
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
    let initial_value = parse_bool_output(assert.get_output(), Some(true))
        .expect("invalid initial configuration bool value");
    // try to set config opposite value
    cmd = Command::cargo_bin("holium-cli").unwrap();
    assert = cmd
        .current_dir(repo_path)
        .arg("config")
        .arg(config_key)
        .arg(toml::Value::Boolean(!initial_value).to_string())
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
    let new_value =
        parse_bool_output(assert.get_output(), None).expect("invalid configuration bool value");
    // check that the configuration has been changed
    assert_ne!(initial_value, new_value)
}