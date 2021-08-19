use std::collections::HashMap;
use std::convert::TryFrom;
use std::{env, fs};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use cid::Cid;
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};
use thiserror::Error;

use crate::utils::PROJECT_DIR;
use crate::utils::storage::{cid_to_object_path, RepoStorage};
use crate::utils::storage::StorageError::FailedToParseCid;

/// `transformation` `remove` command
pub(crate) fn remove_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("rm")
        .alias("remove")
        .about("Remove transformation objects from a repository")
        .arg(
            Arg::with_name("cid")
                .help("The CID of the transformation object to remove")
                .required(true)
                .multiple(true)
        )
}

/// `transformation` `remove` command handler
pub(crate) fn handle_remove_cmd(matches: &ArgMatches) -> Result<()> {
    Ok(())
}
