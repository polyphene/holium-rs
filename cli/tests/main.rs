mod repo;

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn cli_is_callable() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn cannot_unsafely_init_twice() {
    // init a repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    fs::create_dir(&temp_dir.join(".holium")).unwrap();
    // try to unsafely initialize the repository again
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to initiate"))
        .stderr(predicate::str::contains("force"));
}

#[test]
fn can_init_twice_with_the_force_option() {
    // init a repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    fs::create_dir(&temp_dir.join(".holium")).unwrap();
    // try to initialize the repository again with the force option
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--force")
        .assert()
        .success();
}

#[test]
fn init_cmd_creates_project_structure() {
    // initialize a repository
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(temp_dir.path())
        .arg("init")
        .assert();
    // check success message
    assert
        .success()
        .stdout(predicate::str::contains("Initialized Holium repository."));
    // check that the project structure has been created
    let local_holium_path = temp_dir.join(".holium");
    assert!(local_holium_path.join("cache").exists());
    assert!(local_holium_path.join("config").exists());
    assert!(local_holium_path.join("config.local").exists());
    assert!(local_holium_path.join("objects").exists());
}
