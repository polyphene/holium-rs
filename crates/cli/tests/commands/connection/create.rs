use assert_cmd::Command;
use predicates::prelude::predicate;
use crate::helpers::connection::{build_connection_create_cmd, build_connection_id, default_connection_id, node_type_name_alternative_pairs, node_type_name_pairs, NON_VALID_SELECTOR, NON_VALID_TYPE, SELECTOR, setup_repo_with_all_node_types, SHAPER_TYPE, SOURCE_TYPE, TRANSFORMATION_TYPE};
use crate::helpers::shaper::SHAPER_ALTERNATIVE_NAME;
use crate::helpers::source::{SOURCE_ALTERNATIVE_NAME, SOURCE_NAME};
use crate::helpers::transformation::{TRANSFORMATION_ALTERNATIVE_NAME, TRANSFORMATION_NAME};

#[test]
fn help_available() {
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .arg("connection")
        .arg("create")
        .arg("--help")
        .assert();
    // Check success
    assert.success();
}


#[test]
fn cannot_create_connection_without_any_positional_arg() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection without positional argument
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("create")
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--tail-type"))
        .stderr(predicate::str::contains("--tail-name"))
        .stderr(predicate::str::contains("--tail-selector"))
        .stderr(predicate::str::contains("--head-type"))
        .stderr(predicate::str::contains("--head-name"))
        .stderr(predicate::str::contains("--head-selector"));
}

#[test]
fn cannot_create_connection_without_tail_type() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection without tail type
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("create")
        .arg("--tail-name")
        .arg(SOURCE_NAME)
        .arg("--tail-selector")
        .arg(SELECTOR)
        .arg("--head-type")
        .arg(TRANSFORMATION_TYPE)
        .arg("--head-name")
        .arg(TRANSFORMATION_NAME)
        .arg("--head-selector")
        .arg(SELECTOR)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--tail-type"));
}

#[test]
fn cannot_create_connection_without_tail_name() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection without tail name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("create")
        .arg("--tail-type")
        .arg(SOURCE_TYPE)
        .arg("--tail-selector")
        .arg(SELECTOR)
        .arg("--head-type")
        .arg(TRANSFORMATION_TYPE)
        .arg("--head-name")
        .arg(TRANSFORMATION_NAME)
        .arg("--head-selector")
        .arg(SELECTOR)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--tail-name"));
}

#[test]
fn cannot_create_connection_without_tail_selector() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection without tail selector
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("create")
        .arg("--tail-type")
        .arg(SOURCE_TYPE)
        .arg("--tail-name")
        .arg(SOURCE_NAME)
        .arg("--head-type")
        .arg(TRANSFORMATION_TYPE)
        .arg("--head-name")
        .arg(TRANSFORMATION_NAME)
        .arg("--head-selector")
        .arg(SELECTOR)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--tail-selector"));
}

#[test]
fn cannot_create_connection_without_head_type() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection without head type
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("create")
        .arg("--tail-type")
        .arg(SOURCE_TYPE)
        .arg("--tail-name")
        .arg(SOURCE_NAME)
        .arg("--tail-selector")
        .arg(SELECTOR)
        .arg("--head-name")
        .arg(TRANSFORMATION_NAME)
        .arg("--head-selector")
        .arg(SELECTOR)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--head-type"));
}

#[test]
fn cannot_create_connection_without_head_name() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection without head name
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("create")
        .arg("--tail-type")
        .arg(SOURCE_TYPE)
        .arg("--tail-name")
        .arg(SOURCE_NAME)
        .arg("--tail-selector")
        .arg(SELECTOR)
        .arg("--head-type")
        .arg(TRANSFORMATION_TYPE)
        .arg("--head-selector")
        .arg(SELECTOR)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--head-name"));
}

#[test]
fn cannot_create_connection_without_head_selector() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection without head selector
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("connection")
        .arg("create")
        .arg("--tail-type")
        .arg(SOURCE_TYPE)
        .arg("--tail-name")
        .arg(SOURCE_NAME)
        .arg("--tail-selector")
        .arg(SELECTOR)
        .arg("--head-type")
        .arg(TRANSFORMATION_TYPE)
        .arg("--head-name")
        .arg(TRANSFORMATION_NAME)
        .assert();

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"))
        .stderr(predicate::str::contains("--head-selector"));
}

#[test]
fn cannot_create_connection_with_non_valid_tail_type() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection with non valid tail type
    let assert = build_connection_create_cmd(
        repo_path,
        NON_VALID_TYPE,
        SOURCE_NAME,
        SELECTOR,
        TRANSFORMATION_TYPE,
        TRANSFORMATION_NAME,
        SELECTOR
    );

    // check output
    assert
        .failure()
        .stderr(
            predicate::str::contains(
                format!("\'{}\' isn\'t a valid value for \'--tail-type <TYPE>\'", NON_VALID_TYPE)
            )
        );
}

#[test]
fn cannot_create_connection_with_non_valid_head_type() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();
    // try to create connection with non valid tail type
    let assert = build_connection_create_cmd(
        repo_path,
        SOURCE_TYPE,
        SOURCE_NAME,
        SELECTOR,
        NON_VALID_TYPE,
        TRANSFORMATION_NAME,
        SELECTOR
    );

    // check output
    assert
        .failure()
        .stderr(
            predicate::str::contains(
                format!("\'{}\' isn\'t a valid value for \'--head-type <TYPE>\'", NON_VALID_TYPE)
            )
        );
}

#[test]
fn cannot_create_connection_with_non_existent_tail_node() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();

    // create vec of all possible types and some non existent names
    let node_type_name_alternative_pairs = node_type_name_alternative_pairs();

    for (node_type, node_name) in node_type_name_alternative_pairs {
        // try to create connection with non existing tail node
        let assert = build_connection_create_cmd(
            repo_path,
            node_type,
            node_name,
            SELECTOR,
            TRANSFORMATION_TYPE,
            TRANSFORMATION_NAME,
            SELECTOR
        );

        // check output
        assert
            .failure()
            .stderr(predicate::str::contains(format!("no {} node found with name", node_type)))
            .stderr(predicate::str::contains(node_name));
    }
}

#[test]
fn cannot_create_connection_with_non_existent_head_node() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();

    // create vec of all possible types and some non existent names
    let node_type_name_alternative_pairs = node_type_name_alternative_pairs();

    for (node_type, node_name) in node_type_name_alternative_pairs {
        // try to create connection with non existing head node
        let assert = build_connection_create_cmd(
            repo_path,
            SOURCE_TYPE,
            SOURCE_NAME,
            SELECTOR,
            node_type,
            node_name,
            SELECTOR
        );

        // check output
        assert
            .failure()
            .stderr(predicate::str::contains(format!("no {} node found with name", node_type)))
            .stderr(predicate::str::contains(node_name));
    }
}

#[test]
fn cannot_create_connection_witn_non_valid_tail_selector() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();

    // try to create connection with non valid tail type
    let assert = build_connection_create_cmd(
        repo_path,
        SOURCE_TYPE,
        SOURCE_NAME,
        NON_VALID_SELECTOR,
        TRANSFORMATION_TYPE,
        TRANSFORMATION_NAME,
        SELECTOR
    );

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid holium selector"));

}

#[test]
fn cannot_create_connection_witn_non_valid_head_selector() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();

    // try to create connection with non valid tail type
    let assert = build_connection_create_cmd(
        repo_path,
        SOURCE_TYPE,
        SOURCE_NAME,
        SELECTOR,
        TRANSFORMATION_TYPE,
        TRANSFORMATION_NAME,
        NON_VALID_SELECTOR
    );

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid holium selector"));

}


#[test]
fn cannot_create_connection_with_non_parsable_tail_selector() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();

    // try to create connection with non valid tail type
    let assert = build_connection_create_cmd(
        repo_path,
        SOURCE_TYPE,
        SOURCE_NAME,
        "",
        TRANSFORMATION_TYPE,
        TRANSFORMATION_NAME,
        SELECTOR
    );

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid string can not be parsed to json"));

}

#[test]
fn cannot_create_connection_with_non_parsable_head_selector() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();

    // try to create connection with non valid tail type
    let assert = build_connection_create_cmd(
        repo_path,
        SOURCE_TYPE,
        SOURCE_NAME,
        SELECTOR,
        TRANSFORMATION_TYPE,
        TRANSFORMATION_NAME,
        ""
    );

    // check output
    assert
        .failure()
        .stderr(predicate::str::contains("invalid string can not be parsed to json"));
}

#[test]
fn can_create_connection() {
    // initialize a repository
    let repo = setup_repo_with_all_node_types();
    let repo_path = repo.path();

    // create vec of all possible types and some non existent names
    let node_type_name_pairs = node_type_name_pairs();

    for (tail_node_type, tail_node_name) in node_type_name_pairs.iter() {
        for (head_node_type, head_node_name) in node_type_name_pairs.iter() {
            if tail_node_name != head_node_name {
                // try to create connection with non valid tail type
                let assert = build_connection_create_cmd(
                    repo_path,
                    tail_node_type,
                    tail_node_name,
                    SELECTOR,
                    head_node_type,
                    head_node_name,
                    SELECTOR
                );

                // check output
                assert
                    .success()
                    .stdout(predicate::str::contains("new object created"))
                    .stdout(
                        predicate::str::contains(
                            build_connection_id(
                                tail_node_type,
                                tail_node_name,
                                head_node_type,
                                head_node_name
                            ).as_str()
                        )
                    );
            }
        }
    }
}