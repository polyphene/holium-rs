use assert_cmd::Command;
use crate::transformation::{rm_transformations, add_transformation, ls_transformations};
use predicates::prelude::predicate;
use crate::{setup_repo, import_data};

#[test]
fn help_is_available_for_transformation_remove_cmd() {
    // try to get help for the transformation rm command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("rm").arg("--help").assert();
    // check success
    assert.success();
}

#[test]
fn cannot_remove_transformation_outside_repo() {
    // work in an empty directory
    let temp_dir = assert_fs::TempDir::new().unwrap();
    // try to remove transformation data
    let assert = rm_transformations(temp_dir.path(), vec!["<placeholder>"]);
    // check output
    assert.failure().stderr(predicate::str::contains("inside a Holium repository"));
}

#[test]
fn cannot_remove_unknown_transformation() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to remove unknown data
    let unknown_cid = "bafir4iftd67e5gmel7ij3ao5ltjdekpckftj6km6nwzddoohgfc46rqkpy";
    let mut assert = rm_transformations(repo_path, vec![unknown_cid]);
    // check failure
    assert = assert.failure();
    // check output
    assert = assert.stderr(predicate::str::contains("unknown object identifier"));
    assert.stderr(predicate::str::contains(unknown_cid));
}

#[test]
fn can_remove_transformation_object() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from file
    add_transformation(repo_path, "import.wasm")
        .success();
    // list transformations and get related CID
    let mut assert = ls_transformations(repo_path);
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let initial_cids: Vec<&str> = stdout_str.split_whitespace().collect();
    assert_eq!(initial_cids.len(), 1);
    // remove transformations object
    let assert = rm_transformations(repo_path, initial_cids.clone());
    // check success
    assert.success();
    // list transformations and check final number of transformation objects
    let mut assert = ls_transformations(repo_path);
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let final_cids: Vec<&str> = stdout_str.split_whitespace().collect();
    assert_eq!(final_cids.len(), initial_cids.len() - 1);
}

#[test]
fn can_remove_multiple_transformation_objects_at_once() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from files
    add_transformation(repo_path, "import.wasm")
        .success();
    add_transformation(repo_path, "import_other.wasm")
        .success();
    // list transformations and get related CIDs
    let mut assert = ls_transformations(repo_path);
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let initial_cids: Vec<&str> = stdout_str.split_whitespace().collect();
    assert_eq!(initial_cids.len(), 2);
    // remove transformations objects
    let assert = rm_transformations(repo_path, initial_cids.clone());
    // check success
    assert.success();
    // list transformations and check final number of transformation objects
    let mut assert = ls_transformations(repo_path);
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let final_cids: Vec<&str> = stdout_str.split_whitespace().collect();
    assert_eq!(final_cids.len(), initial_cids.len() - 2);
}

#[test]
fn cannot_remove_data_object_with_transformation_cmd() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from file
    import_data(repo_path, "import_scalar.cbor", "cbor");
    // try to remove data object with transformation command
    let cid = "bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq";
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("transformation")
        .arg("rm")
        .arg(cid)
        .assert();
    // check success
    assert.failure();
}