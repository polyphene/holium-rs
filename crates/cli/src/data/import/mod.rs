use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};

use holium::data::data_tree::Node as DataTreeNode;
use holium::data::linked_data_tree::Node as LinkedDataTreeNode;

use crate::data::DataError;
use crate::utils::PROJECT_DIR;
use crate::utils::storage::RepoStorage;

mod importer;


arg_enum! {
    enum ImportType {
        cbor,
        json,
        bin,
        csv
    }
}

/// `data` `import` command
pub(crate) fn import_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("import")
        .about("Import the content of files")
        .arg(
            Arg::with_name("path")
                .help("The path of the file to import")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::with_name("type")
                .help("Explicitly sets the MIME type of the file to import")
                .takes_value(true)
                .short("t")
                .long("type")
                .possible_values(&ImportType::variants())
                .required(true)
        )
}

/// `data` `import` command handler
pub(crate) fn handle_import_cmd(matches: &ArgMatches) -> Result<()> {
    // Initialize handler context
    let cur_dir = env::current_dir()?;
    let holium_dir = cur_dir.join(PROJECT_DIR);
    let repo_storage = RepoStorage::new(&holium_dir);
    // Get the path of the file to be imported
    let path = Path::new(
        matches
            .value_of("path")
            .context(DataError::FailedToOpenImportFile)?
    );
    // Get the type of the file from CLI argument
    let t = value_t!(matches.value_of("type"), ImportType)
        .context(DataError::InvalidImportFileTypeOptionValue(
            matches.value_of("type").unwrap_or("").to_string())
        )?;
    // Parse the file content into a data tree
    let importer: Box<dyn importer::Importer> = match t {
        ImportType::cbor => Box::new(importer::Cbor()),
        ImportType::json => Box::new(importer::Json()),
        ImportType::bin => Box::new(importer::Bin()),
        ImportType::csv => Box::new(importer::Csv()),
    };
    let importable_value = importer.import(path)?;
    let cbor_value = importable_value.to_cbor();
    let data_tree = DataTreeNode::new(cbor_value);
    // Build linked data tree
    let linked_data_tree = LinkedDataTreeNode::from_data_tree(data_tree)?;
    // Store linked data tree
    let root_object_cid = repo_storage.write_data_tree(&linked_data_tree)?;
    // Print object cid
    println!("{}", &root_object_cid);
    // return
    Ok(())
}