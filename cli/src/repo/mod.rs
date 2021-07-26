//! Interact with a repository of Holium objects stored on the file system.

use std::path::PathBuf;

use anyhow::Result;
use thiserror::Error;
use std::{fs, env};
use std::io::Write;
use std::process::Command;
use console::style;
use crate::utils;
use clap::ArgMatches;
use crate::config::models::ProjectConfig;

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

/// Parses arguments and handles the command.
pub(crate) fn handle_cmd(init_matches: &ArgMatches) -> Result<()> {
    // Get configuration
    let project_config = ProjectConfig::new(None)?;
    // Get path to current directory
    let cur_dir = env::current_dir().unwrap();
    // Initialize a Holium repository in current directory
    let no_scm = init_matches.is_present("no-scm") || project_config.config.core.no_scm;
    let no_dvc = init_matches.is_present("no-dvc") || project_config.config.core.no_dvc;
    let force = init_matches.is_present("force");
    init(&cur_dir, no_scm, no_dvc, force)
}

/// Creates a new empty repository on the given directory, basically creating a `.holium` directory.
///
/// It is recommended to track the repository with a SCM and a data version control tool. Otherwise,
/// the options `--no-scm` and/or `--no-dvc` should be used.
///
/// In case the directory is not empty, the `--force` option must be used in order to override it.
pub fn init(root_dir: &PathBuf, no_scm: bool, no_dvc: bool, force: bool) -> Result<()> {

    // If root directory is already an initialized repository, force re-initialization or throw an error
    let local_holium_path = root_dir.join(utils::PROJECT_DIR);
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
    let holium_dir = root_dir.join(utils::PROJECT_DIR);
    fs::create_dir(&holium_dir)?;
    fs::create_dir(&holium_dir.join(utils::CACHE_DIR))?;
    fs::create_dir(&holium_dir.join(utils::OBJECTS_DIR))?;
    fs::File::create(&holium_dir.join(utils::CONFIG_FILE))?;
    fs::File::create(&holium_dir.join(utils::LOCAL_CONFIG_FILE))?;

    // Add a .gitignore file
    if is_scm_enabled {
        let gitignore_file = fs::File::create(&holium_dir.join(".gitignore"))?;
        writeln!(&gitignore_file, "{}", utils::CACHE_DIR)?;
        writeln!(&gitignore_file, "{}", utils::LOCAL_CONFIG_FILE)?;
    }

    // Run the DVC tool once
    if is_dvc_enabled {
        let output = Command::new("dvc")
            .arg("add")
            .arg(&holium_dir.join(utils::OBJECTS_DIR))
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
