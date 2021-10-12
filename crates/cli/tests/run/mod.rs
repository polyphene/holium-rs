use crate::setup_repo;
use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;
use std::path::Path;

fn build_transformation_add_cmd(repo_path: &Path, transformation_filename: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("run")
        .join("assets")
        .join(transformation_filename);
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("add")
        .arg(file_path)
        .assert();
    assert
}

fn build_run_cmd(repo_path: &Path, transformation_name: &str, input_file: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("run")
        .join("assets")
        .join(input_file);
    let assert = cmd
        .current_dir(repo_path)
        .arg("run")
        .arg(transformation_name)
        .arg(file_path)
        .arg("--type")
        .arg("json")
        .assert();
    assert
}

#[test]
fn help_is_available_for_run_cmd() {
    // try to get help for the data command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("run").arg("--help").assert();
    // check success
    assert.success();
}

#[test]
fn can_run_and_store_data() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();

    // Add transformation
    let assert = build_transformation_add_cmd(repo_path, "module.wasm");
    assert.success();

    // Run
    let mut assert = build_run_cmd(repo_path, "main", "inputs.json");
    assert.success();

    // Fetch data in repo, should contain output
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.current_dir(repo_path).arg("data").arg("ls").assert();
    assert.success().stdout(predicate::str::contains(
        "bafyr4igv7kis76dq7kk7qotuzgyzkr4233pubsbl64rv7me36tjisxz2ti",
    ));
}

#[test]
fn cannot_run_on_non_existant_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();

    // Add transformation
    let assert = build_transformation_add_cmd(repo_path, "module.wasm");
    assert.success();

    // Run
    let mut assert = build_run_cmd(repo_path, "not_here", "inputs.json");
    assert
        .failure()
        .stderr(predicate::str::contains("failed to run transformation"));
}

#[test]
fn cannot_run_on_wrong_format_input() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();

    // Add transformation
    let assert = build_transformation_add_cmd(repo_path, "module.wasm");
    assert.success();

    // Run
    let mut assert = build_run_cmd(repo_path, "main", "wrong_inputs.json");
    assert
        .failure()
        .stderr(predicate::str::contains("failed to run transformation"));
}
