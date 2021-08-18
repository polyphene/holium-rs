use assert_cmd::Command;
use crate::setup_repo;

#[test]
fn help_is_available_for_data_remove_cmd() {
    // try to get help for the data remove command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("data").arg("rm").arg("--help").assert();
    // check success
    assert.success();
}

// TODO check thar error when not in repo

// TODO check error message when provided path is not a valid CID

// TODO parsing of argument, base-32, should be case-insensitive

// TODO check error message when trying to remove unknown file

// TODO should be able to remove one file (check w/ ls)

// TODO should be able to remove multiple files at once (check w/ ls)




// TODO check that combined commands works to remove all objects from individual calls
