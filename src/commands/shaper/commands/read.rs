use crate::utils::errors::Error::{
    BinCodeDeserializeFailed, DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::models::shaper::Shaper;
use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("read")
        .about("Read a node")
        .args(&[Arg::with_name("name")
            .help("Name of the node")
            .required(true)
            .value_name("NAME")])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let name = matches
        .value_of("name")
        .context(MissingRequiredArgument("name".to_string()))?;
    // get object from local database
    let encoded = local_context
        .shapers
        .get(name)
        .context(DbOperationFailed)?
        .ok_or(NoObjectForGivenKey(name.to_string()))?;
    let mut decoded: Shaper = bincode::deserialize(&encoded[..])
        .ok()
        .context(BinCodeDeserializeFailed)?;
    decoded.name = name.to_string();
    // print
    Shaper::table_print(vec![&decoded]);
    Ok(())
}
