//! Interact with a repository of Holium objects stored on the file system.

use std::path::PathBuf;

use anyhow::Result;
use thiserror::Error;

/// The name of the directory where all data related to the Holium Framework in a repository is stored.
const HOLIUM_DIR_NAME: &'static str = ".holium";

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

    // Initialize variables
    let local_holium_path = root_dir.join(HOLIUM_DIR_NAME);

    // Check if root directory is already an initialized repository
    let is_holium_dir = local_holium_path.exists();

    if !force && is_holium_dir {
        return Err(RepoError::RepoAlreadyInitialized.into());
    }


    Ok(())
}
