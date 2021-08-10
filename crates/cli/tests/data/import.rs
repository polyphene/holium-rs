use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::predicate;

use crate::setup_repo;

#[test]
fn help_is_available_for_data_import_cmd() {
    // try to get help for the data import command
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd.arg("data").arg("import").arg("--help").assert();
    // check success
    assert.success();
}

/// Given the CID of an object, check if the given object is stored at its due place in a repository.
fn object_file_exists(repo_path: &Path, object_cid_str: &str) -> bool {
    let (cid_prefix, cid_suffix) = object_cid_str.split_at(9);
    let expected_path = repo_path
        .join(".holium")
        .join("objects")
        .join(cid_prefix)
        .join(cid_suffix);
    expected_path.exists()
}

fn test_import_helper(file_name: &str, file_type_arg: &str, expected_root_cid_str: &str, other_expected_cid_strs: Vec<&str>) {
    // initialize a repository
    let repo = setup_repo();
    let repo_path = repo.path();
    // check that the data files we expect to create originally do not exist
    assert!(!object_file_exists(repo_path, expected_root_cid_str));
    for cid in &other_expected_cid_strs {
        assert!(!object_file_exists(repo_path, cid));
    }
    // try to import data from file
    let original_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("assets")
        .join(file_name);
    let mut cmd = Command::cargo_bin("holium-cli").unwrap();
    let assert = cmd
        .current_dir(repo_path)
        .arg("data")
        .arg("import")
        .arg("--type")
        .arg(file_type_arg)
        .arg(original_file_path)
        .assert();
    // check success message
    assert
        .success()
        .stdout(predicate::str::contains(expected_root_cid_str));
    // check that expected files have been created
    assert!(object_file_exists(repo_path, expected_root_cid_str));
    for cid in &other_expected_cid_strs {
        assert!(object_file_exists(repo_path, cid));
    }
}

#[test]
fn can_import_cbor_data_file_scalar() {
    test_import_helper(
        "import_scalar.cbor",
        "cbor",
        "bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq",
        vec![]
    );
}

#[test]
fn can_import_cbor_data_file_recursive() {
    test_import_helper(
        "import_recursive.cbor",
        "cbor",
        "bafyr4iaboxtdci2fq5i65vzoe4jzjeqsafdmh46mz6qzhyfszd4ocealaa",
        vec!["bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq"]
    );
}

#[test]
fn can_import_json_data_file() {
    test_import_helper(
        "import.json",
        "json",
        "bafyr4iaboxtdci2fq5i65vzoe4jzjeqsafdmh46mz6qzhyfszd4ocealaa",
        vec![]
    );
}

#[test]
fn can_import_data_file_twice() {
    // file we try to import includes several times the same nodes
    test_import_helper(
        "import_with_duplicate.json",
        "json",
        "bafyr4ieqatdcyphkdor6farlisuliksatg2hfvirliy23tgbobq2ed3on4",
        vec!["bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq"]
    );
}

#[test]
fn can_import_bin_data_file() {
    test_import_helper(
        "import.bin",
        "bin",
        "bafir4ibosdcgzkas225gir76lcfm5ijtb4auhg7w6e5rih4e2l6yaz62ka",
        vec![]
    );
}

#[test]
fn can_import_csv_data_file() {
    test_import_helper(
        "import.csv",
        "csv",
        "bafyr4ifdnpmbxr6ikms7ksoeyiz32gzinpo3yyeohjj3rovbkb4jnufuzi",
        vec![
            "bafyr4ia3nxsms65abug5ivmzpgdp44ra2e2cecgaxl4bqrnpcnacrmnite",
            "bafyr4ibelcqzk2a5i3x7j5u3scko66zeovldlwdqtku2rw5th6efeplgvi",
            "bafyr4ifdnpmbxr6ikms7ksoeyiz32gzinpo3yyeohjj3rovbkb4jnufuzi",
        ]
    );
}