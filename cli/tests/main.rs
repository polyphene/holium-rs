use assert_cmd::Command;

#[test]
fn cli_is_callable() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    cmd.assert().success();
}
