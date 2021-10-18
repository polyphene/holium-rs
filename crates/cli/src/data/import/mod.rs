use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{arg_enum, value_t, App, Arg, ArgMatches, SubCommand};

use holium::data::data_tree::Node as DataTreeNode;
use holium::data::linked_data_tree::Node as LinkedDataTreeNode;

use crate::data::DataError;
use crate::utils::storage::RepoStorage;
use crate::utils::PROJECT_DIR;

mod importer;

// Pub enum here as clap does not handle pub(crate)
arg_enum! {
    pub enum ImportType {
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
                .required(true),
        )
        .arg(
            Arg::with_name("type")
                .help("Explicitly sets the MIME type of the file to import")
                .takes_value(true)
                .short("t")
                .long("type")
                .possible_values(&ImportType::variants())
                .required(true),
        )
}

/// `data` `import` command handler
pub(crate) fn handle_import_cmd(matches: &ArgMatches) -> Result<()> {
    // Get the path of the file to be imported
    let path = Path::new(
        matches
            .value_of("path")
            .context(DataError::FailedToOpenImportFile)?,
    );
    // Get the type of the file from CLI argument
    let t = matches_to_import_type(matches)?;
    // Generate data node
    let data_tree = file_to_data_tree(path, t)?;
    // Build & store linked data tree
    let root_object_cid = store_data_tree(data_tree)?;
    // Print object cid
    println!("{}", &root_object_cid);
    // return
    Ok(())
}

/// [matches_to_import_type] tries to match on a clap [ArgMatches] to return an [ImportType]
pub(crate) fn matches_to_import_type(matches: &ArgMatches) -> Result<ImportType> {
    Ok(value_t!(matches.value_of("type"), ImportType).context(
        DataError::InvalidImportFileTypeOptionValue(
            matches.value_of("type").unwrap_or("").to_string(),
        ),
    )?)
}

/// [file_to_data_tree] takes a path in the Holium objects to generate a new [DataTreeNode]
pub(crate) fn file_to_data_tree(file_path: &Path, import_type: ImportType) -> Result<DataTreeNode> {
    // Parse the file content into a data tree
    let importer: Box<dyn importer::Importer> = match import_type {
        ImportType::cbor => Box::new(importer::Cbor()),
        ImportType::json => Box::new(importer::Json()),
        ImportType::bin => Box::new(importer::Bin()),
        ImportType::csv => Box::new(importer::Csv()),
    };
    let importable_value = importer.import(file_path)?;
    let cbor_value = importable_value.to_cbor();
    Ok(DataTreeNode::new(cbor_value))
}

pub(crate) fn store_data_tree(data_tree: DataTreeNode) -> Result<String> {
    // Initialize context handler
    let repo_storage = RepoStorage::from_cur_dir()?;
    // Build linked data tree
    let linked_data_tree = LinkedDataTreeNode::from_data_tree(data_tree)?;
    // Store linked data tree
    Ok(repo_storage.write_data_tree(&linked_data_tree)?)
}
