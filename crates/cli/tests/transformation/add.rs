use assert_cmd::Command;

use crate::setup_repo;

#[test]
fn help_is_available_for_transformation_add_cmd() {
    // try to get help for the transformation add command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("add").arg("--help").assert();
    // check success
    assert.success();
}

// TODO fail outside repo

// TODO fail if no param

// TODO if no dvc and large file, warn / same with data import

// TODO detect missing WASM magic number (unitary test as well)

// TODO can import successfully ; creates one file (and only one), with right cid (unitary test as well), and untouched (same hash)

// TODO can import twice, warn but ok
