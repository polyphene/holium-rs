use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use std::str::from_utf8;
use crate::utils::local::trees::transformation::Transformation;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list")
        .about("List all nodes of this type")
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get objects from local database
    for iter in &local_context.transformations {
        let (name_vec, encoded) = iter
            .context(DbOperationFailed)?;
        let name = from_utf8(name_vec.as_ref())?;
        let decoded: Transformation = bincode::deserialize(&encoded[..])
            .ok()
            .context(BinCodeDeserializeFailed)?;
        println!("{}: \"{}\"", name, decoded.handle);
    }
    // print
    println!("ok TODO");
    Ok(())
}
