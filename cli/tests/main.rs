mod repo;

use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn cli_is_callable() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn cannot_init_twice() {
    // init a repository manually
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let input_file = temp_dir.child(".holium");
    input_file.touch().unwrap();
    // try to initialize the repository again
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to initiate"))
        .stderr(predicate::str::contains("force"));
}
