//! Interact with a repository of Holium objects stored on the file system.

use std::path::PathBuf;

use anyhow::Result;
use thiserror::Error;
use std::fs;
use std::io::Write;
use std::process::Command;
use console::style;

/// The name of the directory where all data related to the Holium Framework in a repository is stored.
const PROJECT_DIR: &'static str = ".holium";
const CACHE_DIR: &'static str = "cache";
const OBJECTS_DIR: &'static str = "objects";
const CONFIG_FILE: &'static str = "config";
const LOCAL_CONFIG_FILE: &'static str = "config.local";

#[derive(Error, Debug)]
/// Errors for the repo module.
enum RepoError {
    /// Thrown when trying to initialize a repository twice, without the force option.
    #[error("failed to initiate as '.holium' already exists. Use `-f` to force.")]
    AlreadyInitialized,
    /// Thrown when trying to initialize a repository that is not tracked by any supported SCM tool, without the dedicated option.
    #[error("failed to initiate as current repository is not tracked by any SCM tool. Use `--no-scm` to initialize anyway.")]
    NotScmTracked,
    /// Thrown when trying to initialize a repository that is not tracked by any supported DVC tool, without the dedicated option.
    #[error("failed to initiate as current repository is not tracked by any DVC tool. Use `--no-dvc` to initialize anyway.")]
    NotDvcTracked,
    /// Thrown when the process running git exits with an error code.
    #[error("failed to run git")]
    FailedToRunGit,
    /// Thrown when the process running dvc exits with an error code.
    #[error("failed to run dvc")]
    FailedToRunDvc,
}

/// Creates a new empty repository on the given directory, basically creating a `.holium` directory.
///
/// It is recommended to track the repository with a SCM and a data version control tool. Otherwise,
/// the options `--no-scm` and/or `--no-dvc` should be used.
///
/// In case the directory is not empty, the `--force` option must be used in order to override it.
pub fn init(root_dir: &PathBuf, no_scm: bool, no_dvc: bool, force: bool) -> Result<()> {

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
            return Err(RepoError::AlreadyInitialized.into());
        }
    }

    // Check if the repository is tracked with an SCM and/or a Data Version Control tool
    let is_scm_enabled = root_dir.join(".git").exists();
    let is_dvc_enabled = root_dir.join(".dvc").exists();

    // Enforce usage with an SCM and/or a Data Version Control tool, or with appropriate forcing options
    verify_scm_and_dvc_usage(is_scm_enabled, is_dvc_enabled, no_scm, no_dvc)?;

    // Create project structure
    create_project_structure(&root_dir, is_scm_enabled, is_dvc_enabled)?;

    Ok(())
}

fn create_project_structure(root_dir: &PathBuf, is_scm_enabled: bool, is_dvc_enabled: bool) -> Result<()> {
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
        let output = Command::new("dvc")
            .arg("add")
            .arg(&holium_dir.join(OBJECTS_DIR))
            .output()?;
        if !output.status.success() {
            return Err(RepoError::FailedToRunDvc.into());
        }
    }

    // Run the SCM tool once
    if is_scm_enabled {
        let output = Command::new("git")
            .arg("add")
            .arg(&holium_dir)
            .output()?;
        if !output.status.success() {
            return Err(RepoError::FailedToRunGit.into());
        }
    }

    // Print success message
    println!("Initialized Holium repository.");

    Ok(())
}

fn verify_scm_and_dvc_usage(is_scm_enabled: bool, is_dvc_enabled: bool, no_scm: bool, no_dvc: bool) -> Result<()> {
    if !is_scm_enabled && !no_scm {
        return Err(RepoError::NotScmTracked.into());
    }
    if !is_dvc_enabled && !no_dvc {
        return Err(RepoError::NotDvcTracked.into());
    }
    if is_scm_enabled && !is_dvc_enabled {
        // Warn against the use of SCM with no DVC tool
        println!("{}", style("Initializing a repository without data version control may lead to commit large files.\nConsider using DVC : https://dvc.org/\n").yellow())
    }
    Ok(())
}
