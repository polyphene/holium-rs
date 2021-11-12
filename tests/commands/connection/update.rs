use crate::helpers::connection::{
    build_connection_id, build_connection_read_cmd, default_connection_id,
    node_type_name_alternative_pairs, node_type_name_pairs, setup_repo_with_connection,
    ALTERNATIVE_SELECTOR, NON_VALID_SELECTOR, NON_VALID_TYPE, SELECTOR, SHAPER_TYPE, SOURCE_TYPE,
    TRANSFORMATION_TYPE,
};
use crate::helpers::shaper::SHAPER_ALTERNATIVE_NAME;
use crate::helpers::source::{SOURCE_ALTERNATIVE_NAME, SOURCE_NAME};
use crate::helpers::transformation::{TRANSFORMATION_ALTERNATIVE_NAME, TRANSFORMATION_NAME};
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd.arg("connection").arg("update").arg("--help").assert();
    // Check success
    assert.success();
}

#[test]
fn cannot_update_connection_without_id() {
    // initialize a repository
    let repo = setup_repo_with_connection();
    let repo_path = repo.path();
    // try to update connection without tail type
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("update")
        .arg("--tail-selector")
        .arg(ALTERNATIVE_SELECTOR)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains(
            "required arguments were not provided",
        ))
        .stderr(predicate::str::contains("<ID>"));
}

#[test]
fn cannot_update_connection_with_non_valid_tail_selector() {
    // initialize a repository
    let repo = setup_repo_with_connection();
    let repo_path = repo.path();

    // try to update connection with non valid tail type
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("update")
        .arg(default_connection_id().as_str())
        .arg("--tail-selector")
        .arg(NON_VALID_SELECTOR)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid holium selector"));
}

#[test]
fn cannot_update_connection_with_non_valid_head_selector() {
    // initialize a repository
    let repo = setup_repo_with_connection();
    let repo_path = repo.path();

    // try to update connection with non valid tail type
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("update")
        .arg(default_connection_id().as_str())
        .arg("--head-selector")
        .arg(NON_VALID_SELECTOR)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid holium selector"));
}

#[test]
fn cannot_update_connection_with_non_parsable_tail_selector() {
    // initialize a repository
    let repo = setup_repo_with_connection();
    let repo_path = repo.path();

    // try to update connection with non valid tail type
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("update")
        .arg(default_connection_id().as_str())
        .arg("--tail-selector")
        .arg("")
        .assert();

    // check output
    assert.failure().stderr(predicate::str::contains(
        "invalid string can not be parsed to json",
    ));
}

#[test]
fn cannot_update_connection_with_non_parsable_head_selector() {
    // initialize a repository
    let repo = setup_repo_with_connection();
    let repo_path = repo.path();

    // try to update connection with non valid tail type
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("update")
        .arg(default_connection_id().as_str())
        .arg("--head-selector")
        .arg("")
        .assert();

    // check output
    assert.failure().stderr(predicate::str::contains(
        "invalid string can not be parsed to json",
    ));
}

#[test]
fn can_update_connection_without_any_positional_arg() {
    // initialize a repository
    let repo = setup_repo_with_connection();
    let repo_path = repo.path();
    // try to update connection without positional argument
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("update")
        .arg(default_connection_id().as_str())
        .assert();

    // check output
    assert
        .success()
        .stdout(predicate::str::contains("object updated"))
        .stdout(predicate::str::contains(default_connection_id().as_str()));
}

#[test]
fn can_update_connection() {
    // initialize a repository
    let repo = setup_repo_with_connection();
    let repo_path = repo.path();

    // try to update connection
    let mut cmd = Command::cargo_bin("holium").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("update")
        .arg(default_connection_id().as_str())
        .arg("--tail-selector")
        .arg(ALTERNATIVE_SELECTOR)
        .arg("--head-selector")
        .arg(ALTERNATIVE_SELECTOR)
        .assert();

    // check output
    assert
        .success()
        .stdout(predicate::str::contains("object updated"))
        .stdout(predicate::str::contains(default_connection_id().as_str()));

    // try to read connection
    let assert = build_connection_read_cmd(repo_path, default_connection_id().as_str());
    // check output
    assert
        .success()
        .stdout(predicate::str::contains(default_connection_id().as_str()))
        .stdout(predicate::str::contains("\"i\""))
        .stdout(predicate::str::contains("\">\""));
}
