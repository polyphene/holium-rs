use assert_cmd::Command;

#[test]
fn help_is_available_for_transformation_list_cmd() {
    // try to get help for the transformation ls command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("ls").arg("--help").assert();
    // check success
    assert.success();
}

// TODO cannot list outside repo

// TODO can list empty repo

// TODO can list when data only, no transfo (unitary test as well to differentiate files (data, transfo bytecode and unknown)

// TODO can list when one transformation (check format)

// TODO can list when two transformations (check format)
