use assert_cmd::Command;

#[test]
fn help_is_available_for_transformation_remove_cmd() {
    // try to get help for the transformation rm command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("rm").arg("--help").assert();
    // check success
    assert.success();
}

// TODO cannot remove outside repo

// TODO cannot remove unknown file (TODO)

// TODO can remove one transformation

// TODO can remove multiple transformations at once

// TODO cannot remove data with transfo command (warn)
