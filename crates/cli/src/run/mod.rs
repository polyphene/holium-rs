//! CLI command to run wasm modules in a Holium Repository

use crate::utils::storage::{cid_to_object_path, RepoStorage};
use anyhow::{Context, Result};
use clap::{arg_enum, value_t, App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use thiserror::Error;

use holium::runtime::*;

use crate::data::import::{file_to_data_tree, matches_to_import_ty, store_data_tree, ImportType};
use holium::data::data_tree::Node as DataTreeNode;
use holium_utils::cbor::WASM_MAGIC_NUMBER;

#[derive(Error, Debug)]
/// Errors for run operation.
pub(crate) enum RunError {
    /// Thrown when there a deser problem is raised on data
    #[error("deser error while handling data")]
    DeserError,
    /// Thrown when the bytecode does not seem to be wasm
    #[error("file is not a valid wasm bytecode")]
    NonValidWasmBytecode,
    /// Thrown when failing to read a module file
    #[error("failed to read module file")]
    FailedToReadTransformationFile,
    /// Thrown when failing to instantiate a module bytecode
    #[error("failed to instantiate module bytecode")]
    FailedToInstantiateModuleBytecode,
    /// Thrown when failing to run transformation
    #[error("failed to run transformation")]
    FailedToRunTransformation,
    /// Thrown when failing to open a file in order to import it
    #[error("failed to open file requested for import")]
    FailedToOpenImportFile,
}

/// `data` command
pub(crate) fn run_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("run")
        .about("Execute pipelines on a given data file")
        .arg(
            Arg::with_name("transformation")
                .help("Name of the transformation to execute")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("path")
                .help("The path of the file to import")
                .index(2)
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

/// `run` command handler
pub(crate) fn handle_cmd(run_matches: &ArgMatches) -> Result<()> {
    // Initialize context handler
    let repo_storage = RepoStorage::from_cur_dir()?;
    // Get the path of the file to be imported
    let path = Path::new(
        run_matches
            .value_of("path")
            .context(RunError::FailedToOpenImportFile)?,
    );
    // Get the type of the file from CLI argument
    let t = matches_to_import_ty(run_matches)?;
    let data_tree = file_to_data_tree(path, t)?;
    // Data tree to bytes
    let serialized_data_tree = serde_cbor::to_vec(&data_tree).context(RunError::DeserError)?;
    // Build & store linked data tree
    let root_object_cid = store_data_tree(data_tree)?;

    // Get the name of the transformation to be executed
    let transformation = run_matches.value_of("transformation").unwrap();

    // Instantiate runtime
    let mut runtime = Runtime::new()?;

    println!("Runtime initialized");

    // Get all transformation CIDs
    for cid in repo_storage.transformation_cids.iter() {
        // Retrieve transformation path
        let full_path = repo_storage.root.join(cid_to_object_path(cid));
        // Get wasm bytecode
        let bytecode =
            bytecode_from_file(full_path).context(RunError::FailedToReadTransformationFile)?;

        // Instantiate bytecode
        runtime
            .instantiate(&bytecode)
            .context(RunError::FailedToInstantiateModuleBytecode)?;

        println!("Running {}", cid);

        // Run transformation
        let payload_bytes = runtime
            .run(transformation, &serialized_data_tree)
            .context(RunError::FailedToRunTransformation)?;

        // Deserialize return payload
        let payload: DataTreeNode =
            serde_cbor::from_slice(&payload_bytes).context(RunError::DeserError)?;

        // Build & store return payload
        let root_object_cid = store_data_tree(payload)?;
        println!("Input data: {}", root_object_cid);
        println!("Output data: {}", root_object_cid);
    }

    Ok(())
}

fn bytecode_from_file(file_path: PathBuf) -> Result<Vec<u8>> {
    let mut f = File::open(&file_path)?;
    let metadata = std::fs::metadata(&file_path)?;
    let mut buffer = vec![0; metadata.len() as usize];

    f.read(&mut buffer).unwrap();

    if &buffer[1..5] == WASM_MAGIC_NUMBER {
        buffer = buffer[1..].to_vec();
    } else if &buffer[2..6] == WASM_MAGIC_NUMBER {
        buffer = buffer[2..].to_vec();
    } else if &buffer[3..7] == WASM_MAGIC_NUMBER {
        buffer = buffer[3..].to_vec();
    } else if &buffer[5..9] == WASM_MAGIC_NUMBER {
        buffer = buffer[5..].to_vec();
    } else if &buffer[9..13] == WASM_MAGIC_NUMBER {
        buffer = buffer[9..].to_vec();
    } else {
        return Err(RunError::NonValidWasmBytecode.into());
    }

    Ok(buffer)
}
