use assert_cmd::Command;

mod add;
mod list;
mod remove;

#[test]
fn help_is_available_for_transformation_cmd() {
    // try to get help for the transformation command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("--help").assert();
    // check success
    assert.success();
}
