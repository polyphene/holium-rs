use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::env;
use crate::utils::repo::paths::HOLIUM_DIR;
use crate::utils::repo::errors::Error::OutsideHoliumRepo;

pub fn get_root_path() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    loop {
        if dir.join(HOLIUM_DIR).exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err(OutsideHoliumRepo.into())
        }
    }
}