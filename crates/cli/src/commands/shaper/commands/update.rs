use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{
    BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::helpers::jsonschema::validate_json_schema;
use crate::utils::local::helpers::prints::commands_outputs::print_update_success;
use crate::utils::local::models::shaper::{OptionalShaper, Shaper};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Update a node")
        .args(&[
            Arg::with_name("name")
                .help("Name of the node")
                .required(true)
                .value_name("NAME"),
            Arg::with_name("json-schema")
                .help("JSON Schema of the node")
                .takes_value(true)
                .value_name("JSON-SCHEMA")
                .long("json-schema"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let name = matches
        .value_of("name")
        .context(MissingRequiredArgument("name".to_string()))?;
    let json_schema = matches.value_of("json-schema");
    // check that the object exists
    if !local_context
        .shapers
        .contains_key(name)
        .context(DbOperationFailed)?
    {
        return Err(NoObjectForGivenKey(name.to_string()).into());
    }
    // validate JSON schema, if any
    if let Some(json_schema) = json_schema {
        validate_json_schema(json_schema)?;
    }
    // merge object
    let merge_shaper = OptionalShaper {
        name: None,
        json_schema: json_schema.map(|s| s.to_string()),
    };
    let merge_shaper_encoded = bincode::serialize(&merge_shaper).context(BinCodeSerializeFailed)?;
    local_context
        .shapers
        .merge(name, merge_shaper_encoded)
        .context(DbOperationFailed)?;
    print_update_success(name);
    Ok(())
}
