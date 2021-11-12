use crate::helpers::repo::setup_repo;
use crate::helpers::source::{
    build_source_create_cmd, build_source_read_cmd, JSON_SCHEMA, NON_VALID_JSON_SCHEMA, SOURCE_NAME,
};
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("source").arg("create").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_create_source_outside_repo() {
    // work in an empty directory
    let temp_dir = assert_fs::TempDir::new().unwrap();
    // try to create source
    let assert = build_source_create_cmd(temp_dir.path(), SOURCE_NAME, JSON_SCHEMA);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("inside a Holium repository"));
}

#[test]
fn cannot_create_source_without_any_positional_arg() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create source without positional argument
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("create")
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains(
            "required arguments were not provided",
        ))
        .stderr(predicate::str::contains("<NAME>"))
        .stderr(predicate::str::contains("--json-schema"));
}

#[test]
fn cannot_create_source_without_name() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create create source without name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("create")
        .arg("--json-schema")
        .arg(JSON_SCHEMA)
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains(
            "required arguments were not provided",
        ))
        .stderr(predicate::str::contains("<NAME>"));
}

#[test]
fn cannot_create_source_without_json_schema() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create source without json schema
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("source")
        .arg("create")
        .arg(SOURCE_NAME)
        .assert();
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains(
            "required arguments were not provided",
        ))
        .stderr(predicate::str::contains("--json-schema"));
}

#[test]
fn cannot_create_source_with_non_valid_json_schema() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create source w/ non valid json schema
    let assert = build_source_create_cmd(repo_path, SOURCE_NAME, NON_VALID_JSON_SCHEMA);
    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("json schema"));
}

#[test]
fn cannot_create_source_with_non_parsable_json_schema() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create source with empty string
    let assert = build_source_create_cmd(repo_path, SOURCE_NAME, "");
    // check output
    assert.failure().stderr(predicate::str::contains(
        "invalid string can not be parsed to json",
    ));
}

#[test]
fn cannot_create_source() {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // try to create source
    let assert = build_source_create_cmd(repo_path, SOURCE_NAME, JSON_SCHEMA);

    // check output
    assert
        .success()
        .stdout(predicate::str::contains("new object created"));

    // read created source
    let assert = build_source_read_cmd(repo_path, SOURCE_NAME);

    // check output
    assert
        .success()
        .stdout(predicate::str::contains(SOURCE_NAME))
        .stdout(predicate::str::contains("\"type\": \"string\""));
}
