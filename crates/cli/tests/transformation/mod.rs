use assert_cmd::Command;
use std::path::Path;
use assert_cmd::assert::Assert;

mod add;
mod list;
mod remove;

#[test]
fn help_is_available_for_transformation_cmd() {
    // try to get help for the transformation command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("--help").assert();
    // check success
    assert.success();
}

/// Helper method using the `transformation add` command to import transformation objects from files
fn add_transformation(repo_path: &Path, file_name: &str) -> Assert {
    let original_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("transformation")
        .join("assets")
        .join(file_name);
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("add")
        .arg(original_file_path)
        .assert();
    assert
}

/// Helper method using the `transformation ls` command to list transformations
fn ls_transformations(repo_path: &Path) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("ls")
        .assert();
    assert
}