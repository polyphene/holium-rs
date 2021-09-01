use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use assert_cmd::assert::Assert;
use assert_cmd::Command;
use assert_fs::TempDir;
use cid::Cid;
use predicates::prelude::*;
use walkdir::WalkDir;
use itertools::Itertools;

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

/// Helper method that test the format of an output line, expecting a CID
fn check_output_cid_line_format(line: &str, valid_codecs: Vec<u64>) {
    // check that the string is lowercase
    assert!(line.chars().all(|c| c.is_lowercase() || c.is_numeric()));
    // check that a CID can be built from the string
    let cid = Cid::try_from(line).unwrap();
    // validate the format of the CID
    assert_eq!(cid.version(), cid::Version::V1);
    assert_eq!(cid.hash().code(), 0x1e);
    assert_eq!(cid.hash().size(), 32);
    let cid_codec = cid.codec();
    assert!(valid_codecs.iter().any(|&valid_codec| valid_codec == cid_codec));
    assert!(cid.codec() == 0x51 || cid.codec() == 0x71);
}

// reference : https://stackoverflow.com/a/51272639
fn is_sorted<I>(data: I) -> bool
    where
        I: IntoIterator,
        I::Item: Ord + Clone,
{
    data.into_iter().tuple_windows().all(|(a, b)| a <= b)
}

/// Helper method that checks the format of an output of an object (data, transformation,…) list command.
/// Return the number of output lines.
fn check_output_cid_lines_format(lines_str: &str) -> usize {
    // split into individual lines
    let lines: Vec<&str> = lines_str.split_whitespace().collect();
    // check format of individual lines
    &lines.iter().for_each(|x| check_output_cid_line_format(x, vec![0x51, 0x71]));
    // check uniqueness of each line
    assert!(&lines.iter().all_unique());
    // check that lines are sorted in alphabetical order
    assert!(is_sorted(&lines));
    // return
    lines.len()
}

// Count the number of object files stored in a Holium repository.
// All files stored in the objects tree directory are counted, with little to no additional validation.
fn count_object_files(repo_path: &Path) -> usize {
    WalkDir::new(repo_path.join(".holium").join("objects"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count()
}