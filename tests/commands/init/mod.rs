use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_is_callable() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn cannot_unsafely_init_twice() {
    // initialize a repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    fs::create_dir(&temp_dir.join(".holium")).unwrap();
    // try to unsafely initialize the repository again
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to initiate"))
        .stderr(predicate::str::contains("force"));
}

#[test]
fn can_init_twice_with_the_force_option() {
    // initialize a repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    fs::create_dir(&temp_dir.join(".holium")).unwrap();
    fs::create_dir(&temp_dir.join(".git")).unwrap();
    fs::create_dir(&temp_dir.join(".dvc")).unwrap();
    // try to initialize the repository again with the force option
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("init")
        .arg("--force")
        .assert()
        .success();
}

#[test]
fn cannot_init_with_no_scm_by_default() {
    // initialize a dvc repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    fs::create_dir(&temp_dir.join(".dvc")).unwrap();
    // try to initialize a holium repository
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.current_dir(temp_dir.path()).arg("init").assert();
    // check error message
    assert
        .failure()
        .stderr(predicate::str::contains("failed to initiate"))
        .stderr(predicate::str::contains("--no-scm"));
}

#[test]
fn can_init_with_no_scm_with_option() {
    // initialize a dvc repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    fs::create_dir(&temp_dir.join(".dvc")).unwrap();
    // initialize a holium repository with the `--no-scm` option
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--no-scm")
        .assert();
    // check success
    assert.success();
}

#[test]
fn cannot_init_with_no_dvc_by_default() {
    // initialize a git repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    fs::create_dir(&temp_dir.join(".git")).unwrap();
    // try to initialize a holium repository
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.current_dir(temp_dir.path()).arg("init").assert();
    // check error message
    assert
        .failure()
        .stderr(predicate::str::contains("failed to initiate"))
        .stderr(predicate::str::contains("--no-dvc"));
}

#[test]
fn can_init_with_no_dvc_with_option() {
    // initialize a git repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    fs::create_dir(&temp_dir.join(".git")).unwrap();
    // initialize a holium repository with the `--no-dvc` option
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--no-dvc")
        .assert();
    // check success
    assert.success();
}

#[test]
fn init_cmd_creates_project_structure() {
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
    // check that the project structure has been created
    let local_holium_path = temp_dir.join(".holium");
    assert!(local_holium_path.join("interplanetary").exists());
    assert!(local_holium_path.join("local").exists());
    assert!(local_holium_path.join("portations").exists());
}
