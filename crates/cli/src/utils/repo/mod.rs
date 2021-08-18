//! Module that gathers CLI utility functions related to local repositories

use anyhow::Result;
use std::env;
use crate::utils::PROJECT_DIR;
use crate::utils::CliUtilsError::OutsideHoliumRepo;

/// Verify that current directory, fetched from environment, is inside of a valid repository.
/// Return [Result::Ok] if it is the case, and [Result::Err] otherwise.
pub(crate) fn current_dir_is_valid_repo() -> Result<()> {
    let cur_dir = env::current_dir()?;
    let holium_dir = cur_dir.join(PROJECT_DIR);
    if holium_dir.exists() && holium_dir.is_dir() {
        Ok(())
    } else {
        Err(OutsideHoliumRepo.into())
    }
}
