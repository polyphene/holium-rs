use assert_cmd::Command;

mod create;
mod delete;
mod list;
mod read;
mod update;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("transformation").arg("help").assert();
    // Check success
    assert.success();
}
