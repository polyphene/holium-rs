use crate::helpers::repo::setup_repo;
use crate::helpers::shaper::{
    build_shaper_read_cmd, setup_repo_with_shaper, ALTERNATIVE_JSON_SCHEMA, NON_VALID_JSON_SCHEMA,
    SHAPER_NAME,
};
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("shaper").arg("update").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_update_shaper_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to update shaper without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("update")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn can_update_shaper_without_any_positional_arg() {
    // initialize a repository
    let repo = setup_repo_with_shaper();
    let repo_path = repo.path();

    // try to update shaper without positional argument
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("update")
        .arg(SHAPER_NAME)
        .assert();
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("object updated"));
}

#[test]
fn cannot_update_shaper_with_non_valid_json_schema() {
    // initialize a repository
    let repo = setup_repo_with_shaper();
    let repo_path = repo.path();
    // try to update shaper w/ non valid json schema
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("update")
        .arg(SHAPER_NAME)
        .arg("--json-schema")
        .arg(NON_VALID_JSON_SCHEMA)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("json schema"));
}

#[test]
fn cannot_update_shaper_with_non_parsable_json_schema() {
    // initialize a repository
    let repo = setup_repo_with_shaper();
    let repo_path = repo.path();
    // try to update shaper with empty string
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("update")
        .arg(SHAPER_NAME)
        .arg("--json-schema")
        .arg("")
        .assert();

    // check output
    assert.failure().stderr(predicate::str::contains(
        "invalid string can not be parsed to json",
    ));
}

#[test]
fn can_update_shaper() {
    // initialize a repository
    let repo = setup_repo_with_shaper();
    let repo_path = repo.path();
    // try to update shaper with empty string
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("shaper")
        .arg("update")
        .arg(SHAPER_NAME)
        .arg("--json-schema")
        .arg(ALTERNATIVE_JSON_SCHEMA)
        .assert();

    // check output
    assert.success();

    // read created shaper
    let assert = build_shaper_read_cmd(repo_path, SHAPER_NAME);

    // check output
    assert
        .success()
        .stdout(predicate::str::contains(SHAPER_NAME))
        .stdout(predicate::str::contains("\"type\": \"number\""));
}
