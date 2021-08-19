use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;
use assert_cmd::assert::Assert;
use std::path::{PathBuf, Path};

mod repo;
mod config;
mod data;
mod transformation;


/************************
Test helper functions
 ************************/

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

/// Use the `data import` command to import data objects from files
fn import_data(repo_path: &Path, file_name: &str, file_type: &str) {
    let original_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("assets")
        .join(file_name);
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("import")
        .arg("--type")
        .arg(file_type)
        .arg(original_file_path)
        .assert();
    assert.success();
}