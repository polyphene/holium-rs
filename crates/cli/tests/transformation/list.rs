use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::{setup_repo, check_output_cid_lines_format};
use std::path::Path;
use crate::transformation::{add_transformation, ls_transformations};

#[test]
fn help_is_available_for_transformation_list_cmd() {
    // try to get help for the transformation ls command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("ls").arg("--help").assert();
    // check success
    assert.success();
}

#[test]
fn cannot_list_transformations_outside_repo() {
    // work in an empty directory
    let temp_dir = assert_fs::TempDir::new().unwrap();
    // try to list transformations
    let assert = ls_transformations(temp_dir.path());
    // check output
    assert.failure().stderr(predicate::str::contains("inside a Holium repository"));
}



#[test]
fn cannot_list_data_outside_repo() {
    // work in an empty directory
    let temp_dir = assert_fs::TempDir::new().unwrap();
    // try to list data
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(temp_dir.path())
        .arg("data")
        .arg("ls")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("inside a Holium repository"));
}

#[test]
fn can_list_transformations_in_empty_repository() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // list transformations
    let mut assert = ls_transformations(repo_path);
    // check success
    assert = assert.success();
    // check output format
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let nb_objects = check_output_cid_lines_format(stdout_str.as_str());
    assert_eq!(nb_objects, 0);
}

#[test]
fn can_list_transformations_in_repository_with_objects_data_only() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data object
    let original_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("assets")
        .join("import_recursive.cbor");
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("import")
        .arg("--type")
        .arg("cbor")
        .arg(original_file_path)
        .assert();
    assert.success();
    // list transformations
    assert = ls_transformations(repo_path);
    // check success
    assert = assert.success();
    // check output format
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let nb_objects = check_output_cid_lines_format(stdout_str.as_str());
    assert_eq!(nb_objects, 0);
}

#[test]
fn can_list_transformations_in_repository_with_single_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import transformation object
    let mut assert = add_transformation(repo_path, "import.wasm");
    assert = assert.success();
    let add_cmd_stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let trimmed_add_cmd_stdout_str = add_cmd_stdout_str.trim();
    // list transformations
    assert = ls_transformations(repo_path);
    // check success
    assert = assert.success();
    // check output format
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let nb_objects = check_output_cid_lines_format(stdout_str.as_str());
    stdout_str.contains(trimmed_add_cmd_stdout_str);
    assert_eq!(nb_objects, 1);
}

#[test]
fn can_list_transformations_in_repository_with_multiple_transformations() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import first transformation
    let mut assert = add_transformation(repo_path, "import.wasm");
    assert = assert.success();
    // import second transformation
    assert = add_transformation(repo_path, "import_other.wasm");
    assert = assert.success();
    // list transformations
    assert = ls_transformations(repo_path);
    // check success
    assert = assert.success();
    // check output format
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let nb_objects = check_output_cid_lines_format(stdout_str.as_str());
    assert_eq!(nb_objects, 2);
}
