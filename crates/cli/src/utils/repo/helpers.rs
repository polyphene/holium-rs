use anyhow::{anyhow, Result, Context};
use std::path::{PathBuf, Path};
use std::env;
use thiserror;
use path_clean::PathClean;
use crate::utils::repo::constants::HOLIUM_DIR;
use crate::utils::repo::errors::Error::OutsideHoliumRepo;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("file path outside the project directory")]
    UnsecureFilePath,
    #[error("invalid unicode characters in the file path")]
    NonUnicodeFilePath,
}

/// Pop directories one by one from the current one to find and return the path to the closest
/// project root directory.
pub fn get_root_path() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    loop {
        if dir.join(HOLIUM_DIR).exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err(OutsideHoliumRepo.into());
        }
    }
}

/// Check that a path leads to a location that is part of the project
/// TODO tests required here for security purposes
pub fn to_relative_path_to_project_root(tested_path_os_str: &str) -> Result<String> {
    // Parse path os string and canonicalize it
    let path = absolute_path(PathBuf::from(tested_path_os_str))?;
    // Try to strip root directory path from it. If it fails, then return an error as the location
    // is not part of the project, otherwise return an equivalent path string relative to the
    // project root directory.
    let root_path = get_root_path()?;
    let stripped_path = path.strip_prefix(&root_path)
        .context(Error::UnsecureFilePath)?;
    let stripped_path_str = stripped_path
        .to_str()
        .ok_or(Error::NonUnicodeFilePath)?;
    Ok(stripped_path_str.to_string())
}

/// Get an absolute path string from a possibly relative one, even if the path does not exist.
/// Reference: https://stackoverflow.com/a/54817755 (with light modifications).
pub fn absolute_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();
    Ok(absolute_path)
}