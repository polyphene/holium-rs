use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::predicate;

use crate::{import_data, setup_repo};

// Helper methods that runs the data list command and returns the number of lines in the output
fn get_nb_data_objects_from_data_ls_cmd(repo_path: &Path) -> usize {
    // list data
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("ls")
        .assert();
    // check success
    assert = assert.success();
    // get and use stdout
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let lines: Vec<&str> = stdout_str.split_whitespace().collect();
    lines.len()
}

#[test]
fn help_is_available_for_data_remove_cmd() {
    // try to get help for the data remove command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("data").arg("rm").arg("--help").assert();
    // check success
    assert.success();
}

#[test]
fn argument_is_required_for_data_remove_cmd() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to remove data with no argument
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("rm")
        .assert();
    // check failure
    assert = assert.failure();
    // check output
    assert.stderr(predicate::str::contains("required argument"));
}

#[test]
fn cannot_remove_data_outside_repo() {
    // work in an empty directory
    let temp_dir = assert_fs::TempDir::new().unwrap();
    // try to remove data
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(temp_dir.path())
        .arg("data")
        .arg("rm")
        .arg("<placeholder>")
        .assert();
    // check failure
    assert = assert.failure();
    // check output
    assert.stderr(predicate::str::contains("inside a Holium repository"));
}

#[test]
fn cannot_remove_data_without_providing_a_cid() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to remove data with something other than a CID
    let not_a_cid_str = "NOT_A_CID";
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("rm")
        .arg(not_a_cid_str)
        .assert();
    // check failure
    assert = assert.failure();
    // check output
    assert.stderr(predicate::str::contains("unknown data object identifier"));
}

#[test]
fn cannot_remove_unknown_data() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to remove unknown data
    let unknown_cid = "bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq";
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("rm")
        .arg(unknown_cid)
        .assert();
    // check failure
    assert = assert.failure();
    // check output
    assert = assert.stderr(predicate::str::contains("unknown data object identifier"));
    assert.stderr(predicate::str::contains(unknown_cid));
}

#[test]
fn can_remove_data_object() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from file
    import_data(repo_path, "import_scalar.cbor", "cbor");
    // get number of data objects
    let initial_nb_data_objects = get_nb_data_objects_from_data_ls_cmd(repo_path);
    // remove data object
    let cid = "bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq";
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("rm")
        .arg(cid)
        .assert();
    // check success
    assert.success();
    // check final number of data objects
    let final_nb_data_objects = get_nb_data_objects_from_data_ls_cmd(repo_path);
    assert_eq!(final_nb_data_objects, initial_nb_data_objects - 1);
}

#[test]
fn can_remove_multiple_data_objects_at_once() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from file
    import_data(repo_path, "import_recursive.cbor", "cbor");
    // get number of data objects
    let initial_nb_data_objects = get_nb_data_objects_from_data_ls_cmd(repo_path);
    // remove data object
    let cids = vec![
        "bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq",
        "bafyr4iaboxtdci2fq5i65vzoe4jzjeqsafdmh46mz6qzhyfszd4ocealaa",
    ];
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("rm")
        .arg(cids[0])
        .arg(cids[1])
        .assert();
    // check success
    assert.success();
    // check final number of data objects
    let final_nb_data_objects = get_nb_data_objects_from_data_ls_cmd(repo_path);
    assert_eq!(final_nb_data_objects, initial_nb_data_objects - 2);
}

#[test]
fn cid_argument_is_case_insensitive() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from file
    import_data(repo_path, "import_scalar.cbor", "cbor");
    // remove data object
    let alternating_case_cid = "bAfIr4iDbVg7rB4H75xD5Y52yTlRkWtFiBmAgZaDoMy3oIg3aIeGnR4F3Yq";
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("rm")
        .arg(alternating_case_cid)
        .assert();
    // check success
    assert.success();
}

#[test]
fn removing_all_listable_data_objects_leaves_no_object() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // import data from files
    import_data(repo_path, "import_scalar.cbor", "cbor");
    import_data(repo_path, "import_recursive.cbor", "cbor");
    import_data(repo_path, "import.json", "json");
    import_data(repo_path, "import.csv", "csv");
    // list data
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let mut assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("ls")
        .assert();
    // check success
    assert = assert.success();
    // get and split stdout
    let stdout_str = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let cids: Vec<&str> = stdout_str.split_whitespace().collect();
    // remove all data with one command
    cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("rm");
    for cid in cids {
        cmd.arg(cid);
    }
    assert = cmd.assert();
    // check success
    assert.success();
    // check final number of data objects
    let final_nb_data_objects = get_nb_data_objects_from_data_ls_cmd(repo_path);
    assert_eq!(final_nb_data_objects, 0);
}
