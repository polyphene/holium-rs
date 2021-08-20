use std::path::Path;

use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::predicate;

use crate::{setup_repo, check_output_cid_line_format};
use walkdir::WalkDir;

fn build_transformation_add_cmd(repo_path: &Path, transformation_filename: &str) -> Assert {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("transformation")
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

#[test]
fn help_is_available_for_transformation_add_cmd() {
    // try to get help for the transformation add command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("add").arg("--help").assert();
    // check success
    assert.success();
}

#[test]
fn cannot_add_transformation_outside_repo() {
    // work in an empty directory
    let temp_dir = assert_fs::TempDir::new().unwrap();
    // try to add transformation
    let assert = build_transformation_add_cmd(
        temp_dir.path(), "import.wasm");
    // check output
    assert.failure().stderr(predicate::str::contains("inside a Holium repository"));
}

#[test]
fn cannot_add_transformation_without_any_positional_arg() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation without positional argument
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("add")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("bytecode"));
}

#[test]
fn cannot_add_transformation_which_bytecode_lacks_wasm_magic_number() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to add transformation
    let assert = build_transformation_add_cmd(
        repo_path, "import_corrupted.wasm");
    // check output
    assert.failure().stderr(predicate::str::contains("WebAssembly bytecode"));
}

#[test]
fn can_add_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // count initial number of object files
    let initial_nb_objects = WalkDir::new(repo_path.join("objects")).into_iter().count();
    // add transformation
    let mut assert = build_transformation_add_cmd(
        repo_path, "import.wasm");
    // check output
    assert = assert.success();
    // check final number of files
    let final_nb_objects = WalkDir::new(repo_path.join("objects")).into_iter().count();
    assert_eq!(initial_nb_objects, final_nb_objects-1);
    // check output CID format
    let stdout_string = String::from_utf8_lossy(&assert.get_output().stdout);
    let stdout_str = stdout_string.trim_end();
    check_output_cid_line_format(stdout_str, vec![0x51]);
}


#[test]
fn can_add_twice_the_same_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // count initial number of object files
    let initial_nb_objects = WalkDir::new(repo_path.join("objects")).into_iter().count();
    // add transformation
    let mut assert = build_transformation_add_cmd(
        repo_path, "import.wasm");
    // check output
    assert = assert.success();
    // check intermediate number of files
    let intermediate_nb_objects = WalkDir::new(repo_path.join("objects")).into_iter().count();
    assert_eq!(initial_nb_objects, intermediate_nb_objects -1);
    // add same transformation again
    let mut assert = build_transformation_add_cmd(
        repo_path, "import.wasm");
    // check output
    assert = assert.success();
    // check final number of files
    let final_nb_objects = WalkDir::new(repo_path.join("objects")).into_iter().count();
    assert_eq!(final_nb_objects, intermediate_nb_objects);
}


