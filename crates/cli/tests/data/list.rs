use std::convert::TryFrom;

use assert_cmd::Command;
use cid::Cid;
use itertools::Itertools;
use predicates::prelude::predicate;

use crate::{setup_repo, import_data};
use std::path::Path;
use std::fs;
use predicates::Predicate;

// reference : https://stackoverflow.com/a/51272639
fn is_sorted<I>(data: I) -> bool
    where
        I: IntoIterator,
        I::Item: Ord + Clone,
{
    data.into_iter().tuple_windows().all(|(a, b)| a <= b)
}

/// Helper method that check the format of an output line for the list command
fn check_output_line_format(line: &str) {
    // check that the string is lowercase
    assert!(line.chars().all(|c|c.is_lowercase() || c.is_numeric()));
    // check that a CID can be built from the string
    let cid = Cid::try_from(line).unwrap();
    // validate the format of the CID
    assert_eq!(cid.version(), cid::Version::V1);
    assert_eq!(cid.hash().code(), 0x1e);
    assert_eq!(cid.hash().size(), 32);
    assert!(cid.codec() == 0x51 || cid.codec() == 0x71);
}

/// Helper method that checks the format of an output of the data list command.
/// Return the number of output lines.
fn check_output_lines_format(lines_str: &str) -> usize {
    // split into individual lines
    let lines: Vec<&str> = lines_str.split_whitespace().collect();
    // check format of individual lines
    &lines.iter().for_each(|x| check_output_line_format(x));
    // check uniqueness of each line
    assert!(&lines.iter().all_unique());
    // check that lines are sorted in alphabetical order
    assert!(is_sorted(&lines));
    // return
    lines.len()
}

#[test]
fn help_is_available_for_data_list_cmd() {
    // try to get help for the data ls command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("data").arg("ls").arg("--help").assert();
    // check success
    assert.success();
}

#[test]
fn help_is_available_for_data_list_alis_cmd() {
    // try to get help for the data list command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("data").arg("list").arg("--help").assert();
    // check success
    assert.success();
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
fn can_list_data_objects_in_empty_repository() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // list data
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("ls")
        .assert();
    // check success
    assert = assert.success();
    // check output format
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let nb_objects = check_output_lines_format(stdout_str.as_str());
    assert_eq!(nb_objects, 0);
}

#[test]
fn can_list_data_objects_in_repository_with_one_scalar_object_only() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from file
    import_data(repo_path, "import_scalar.cbor", "cbor");
    // list data
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("ls")
        .assert();
    // check success
    assert = assert.success();
    // check output format
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let nb_objects = check_output_lines_format(stdout_str.as_str());
    assert_eq!(nb_objects, 1);
}

#[test]
fn can_list_data_objects_in_repository_with_recursive_data() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from file
    import_data(repo_path, "import_recursive.cbor", "cbor");
    // list data
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("ls")
        .assert();
    // check success
    assert = assert.success();
    // check output format
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let nb_objects = check_output_lines_format(stdout_str.as_str());
    assert!(1 < nb_objects);
}

#[test]
fn can_list_data_objects_in_spoiled_repository_while_warning() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from file
    import_data(repo_path, "import_scalar.cbor", "cbor");
    // manually spoil the objects local storage directory
    let sup_path = repo_path.join(".holium").join("objects").join("bafibir4d");
    fs::create_dir(&sup_path).unwrap();
    fs::File::create(&sup_path.join("spoiled_file")).unwrap();
    // list data
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("ls")
        .assert();
    // check success
    assert = assert.success();
    // check warning message
    let stderr_str = String::from_utf8_lossy(&assert.get_output().stderr).to_string();
    let predicate_fn = predicate::str::contains("spoiled_file");
    assert!(predicate_fn.eval(stderr_str.as_str()))
}
