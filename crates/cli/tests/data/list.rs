use assert_cmd::Command;
use crate::setup_repo;

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

// TODO specific error when not in repo (specific to the fact that not in repo, but not specific to this command)
// TODO just context (like when parsing config) heading verifier that we are in repo

// TODO check that no line in empty repo

// TODO helper check that all lines unique, all lines right length, and classified in alphabetical order, that they are all CIDs with right version, codec, and format
// TODO returns Result<u8>

// TODO helper on result check that one line for scalar

// TODO check that multiple lines for recursive

// TODO check that if manually polluted repo (the user adds a random file in the `.objects` directory), it still works fine