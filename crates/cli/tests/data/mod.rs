use assert_cmd::Command;

mod import;

#[test]
fn help_is_available_for_data_cmd() {
    // try to get help for the data command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("data").arg("--help").assert();
    // check success
    assert.success();
}
