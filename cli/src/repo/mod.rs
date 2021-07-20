//! Interact with a repository of Holium objects stored on the file system.

use std::path::PathBuf;

use anyhow::Result;
use thiserror::Error;
use std::fs;
use std::io::Write;
use std::process::Command;

/// The name of the directory where all data related to the Holium Framework in a repository is stored.
const PROJECT_DIR: &'static str = ".holium";
const CACHE_DIR: &'static str = "cache";
const OBJECTS_DIR: &'static str = "objects";
const CONFIG_FILE: &'static str = "config";
const LOCAL_CONFIG_FILE: &'static str = "config.local";

#[derive(Error, Debug)]
/// Errors for the repo module.
enum RepoError {
    #[error("failed to initiate, as '.holium' exists. Use `-f` to force.")]
    /// Thrown when trying to initialize a repository twice, without the force option.
    RepoAlreadyInitialized,
}

/// Creates a new empty repository on the given directory, basically creating a `.holium` directory.
///
/// It is recommended to track the repository with a SCM and a data version control tool. Otherwise,
/// the options `--no-scm` and/or `--no-dvc` should be used.
///
/// In case the directory is not empty, the `--force` option must be used in order to override it.
pub fn init(root_dir: &PathBuf, _no_scm: bool, _no_dvc: bool, force: bool) -> Result<()> {

    // If root directory is already an initialized repository, force re-initialization or throw an error
    let local_holium_path = root_dir.join(PROJECT_DIR);
    if local_holium_path.exists() {
        if force {
            if local_holium_path.is_dir() {
                fs::remove_dir_all(local_holium_path)?;
            } else {
                fs::remove_file(local_holium_path)?;
            }
        } else {
            return Err(RepoError::RepoAlreadyInitialized.into())
        }
    }

    create_project_structure(&root_dir)?;

    Ok(())
}

fn create_project_structure(root_dir: &PathBuf) -> Result<()> {
    // Check if the repository is tracked with an SCM and/or a Data Version Control tool
    let is_scm_enabled = root_dir.join(".git").exists();
    let is_dvc_enabled = root_dir.join(".dvc").exists();

    // Warn against the use of SCM with no DVC tool
    if is_scm_enabled && !is_dvc_enabled {
        println!("Initializing a repository without data version control may lead to commit large files.
        Consider using DVC : https://dvc.org/\n")
    }

    // Create project structure
    let holium_dir = root_dir.join(PROJECT_DIR);
    fs::create_dir(&holium_dir)?;
    fs::create_dir(&holium_dir.join(CACHE_DIR))?;
    fs::create_dir(&holium_dir.join(OBJECTS_DIR))?;
    fs::File::create(&holium_dir.join(CONFIG_FILE))?;
    fs::File::create(&holium_dir.join(LOCAL_CONFIG_FILE))?;

    // Add a .gitignore file
    if is_scm_enabled {
        let gitignore_file = fs::File::create(&holium_dir.join(".gitignore"))?;
        writeln!(&gitignore_file, "{}", CACHE_DIR)?;
        writeln!(&gitignore_file, "{}", LOCAL_CONFIG_FILE)?;
    }

    // Run the DVC tool once
    if is_dvc_enabled {
        Command::new("dvc")
            .arg("add")
            .arg(&holium_dir.join(OBJECTS_DIR))
            .output()
            .expect("failed to execute dvc");
    }

    // Run the SCM tool once
    if is_scm_enabled {
        Command::new("git")
            .arg("add")
            .arg(&holium_dir)
            .output()
            .expect("failed to execute git");
    }

    // Print success message
    println!("Initialized Holium repository.");

    Ok(())
}
