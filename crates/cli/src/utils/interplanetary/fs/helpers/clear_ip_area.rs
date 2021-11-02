use anyhow::{Context, Result};
use crate::utils::local::context::LocalContext;
use crate::utils::repo::constants::{HOLIUM_DIR, INTERPLANETARY_DIR};
use std::fs;
use std::io;
use std::io::{Read, Seek};
use cid::Cid;
use std::convert::TryFrom;
use crate::utils::interplanetary::multiformats::compute_cid;
use std::fs::OpenOptions;
use crate::utils::interplanetary::context::InterplanetaryContext;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to clear the interplanetary area")]
    FailedToClear,
}

/// Clear the IP area directory.
pub fn clear_ip_area(ip_context: &InterplanetaryContext) -> Result<()> {
    let ip_area_path = &ip_context.ip_area_path;
    fs::remove_dir_all(&ip_area_path)
        .context(Error::FailedToClear)?;
    fs::create_dir(&ip_area_path)
        .context(Error::FailedToClear)?;
    Ok(())
}

