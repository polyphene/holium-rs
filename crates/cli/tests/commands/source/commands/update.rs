use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::helpers::repo::setup_repo;
use crate::helpers::source::{ALTERNATIVE_JSON_SCHEMA, build_source_read_cmd, NON_VALID_JSON_SCHEMA, setup_repo_with_source, SOURCE_NAME};

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .arg("source")
        .arg("update")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}


#[test]
fn cannot_update_source_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to update source without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("update")
        .assert();
    // check output
    assert.failure().stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn can_update_source_without_any_positional_arg() {
    // initialize a repository
    let repo = setup_repo_with_source();
    let repo_path = repo.path();

    // try to update source without positional argument
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("update")
        .arg(SOURCE_NAME)
        .assert();
    // check output
    assert
        .success()
        .stdout(predicate::str::contains("object updated"));
}

#[test]
fn cannot_update_source_with_non_valid_json_schema() {
    // initialize a repository
    let repo = setup_repo_with_source();
    let repo_path = repo.path();
    // try to update source w/ non valid json schema
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("update")
        .arg(SOURCE_NAME)
        .arg("--json-schema")
        .arg(NON_VALID_JSON_SCHEMA)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("json schema"));
}

#[test]
fn cannot_update_source_with_non_parsable_json_schema() {
    // initialize a repository
    let repo = setup_repo_with_source();
    let repo_path = repo.path();
    // try to update source with empty string
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("update")
        .arg(SOURCE_NAME)
        .arg("--json-schema")
        .arg("")
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid string can not be parsed to json"));
}

#[test]
fn can_update_source() {
    // initialize a repository
    let repo = setup_repo_with_source();
    let repo_path = repo.path();
    // try to update source with empty string
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("update")
        .arg(SOURCE_NAME)
        .arg("--json-schema")
        .arg(ALTERNATIVE_JSON_SCHEMA)
        .assert();

    // check output
    assert.success();

    // read created source
    let assert = build_source_read_cmd(repo_path, SOURCE_NAME);

    // check output
    assert
        .success()
        .stdout(predicate::str::contains(SOURCE_NAME))
        .stdout(predicate::str::contains("\"type\": \"number\""));
}