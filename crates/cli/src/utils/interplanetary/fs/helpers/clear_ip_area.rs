use anyhow::{Context, Result};
use crate::utils::local::context::LocalContext;
use crate::utils::repo::constants::{HOLIUM_DIR, INTERPLANETARY_DIR};
use std::fs;
use std::io;
use std::io::{Read, Seek};
use cid::Cid;
use std::convert::TryFrom;
use crate::utils::interplanetary::multiformats::{compute_cid, cid_to_path};
use std::fs::OpenOptions;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to clear the interplanetary area")]
    FailedToClear,
}

/// Clear the IP area directory.
pub fn clear_ip_area(local_context: &LocalContext) -> Result<()> {
    let ip_area_path = local_context
        .root_path
        .join(HOLIUM_DIR)
        .join(INTERPLANETARY_DIR);
    fs::remove_dir_all(&ip_area_path)
        .context(Error::FailedToClear)?;
    fs::create_dir(&ip_area_path)
        .context(Error::FailedToClear)?;
    Ok(())
}

