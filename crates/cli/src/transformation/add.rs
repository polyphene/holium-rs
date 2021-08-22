use std::env;
use std::fs;
use std::io;
use std::fs::{File, OpenOptions, rename};
use std::io::{BufReader, BufWriter, copy, Read, Seek, SeekFrom, Write};
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use blake3;
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};
use tempfile::{NamedTempFile, tempdir};

use holium_utils::cbor::create_cbor_byte_string_header;

use crate::transformation::TransformationError;
use crate::utils::PROJECT_DIR;
use crate::utils::storage::{RepoStorage, cid_to_object_path};
use holium_utils::multihash::blake3_hash_to_multihash;
use cid::Cid;

/// `transformation` `add` command
pub(crate) fn add_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("add a transformation")
        .arg(
            Arg::with_name("bytecode_path")
                .help("The path of the WebAssembly bytecode to import")
                .index(1)
                .required(true)
        )
}

/// `transformation` `add` command handler
pub(crate) fn handle_add_cmd(matches: &ArgMatches) -> Result<()> {
    // Initialize context handler
    let repo_storage = RepoStorage::from_cur_dir()?;
    // Get the path of the file to be imported
    let original_path = Path::new(
        matches
            .value_of("bytecode_path")
            .context(TransformationError::FailedToOpenImportFile)?
    );
    // Get file metadata
    let original_file_metadata = fs::metadata(original_path)
        .context(TransformationError::FailedToGetFileMetadata)?;
    // Open original file in read-only mode
    let mut original_file = File::open(original_path)
        .context(TransformationError::FailedToOpenImportFile)?;
    // Read for first bytes to validate the presence of WebAssembly 4-byte magic number
    // Reference : https://webassembly.github.io/spec/core/bikeshed/#binary-magic
    const WASM_MAGIC_NUMBER: &[u8; 4] = b"\x00\x61\x73\x6D";
    let mut original_header_buffer = [0u8; 4];
    if original_file.read(&mut original_header_buffer)
        .context(TransformationError::MissingWasmMagicNumber)? < WASM_MAGIC_NUMBER.len()
        || &original_header_buffer[..] != WASM_MAGIC_NUMBER {
        return Err(TransformationError::MissingWasmMagicNumber.into());
    };
    original_file.seek(SeekFrom::Start(0))
        .context(TransformationError::FailedToOpenImportFile)?;
    // Compute CBOR string header
    let payload_len = original_file_metadata.len();
    let cbor_string_header = create_cbor_byte_string_header(payload_len);
    // Create temporary destination file to write to
    let temp_dir = tempdir()
        .context(TransformationError::FailedToCreateImportDestFile)?;
    let temp_path = temp_dir.path().join("bytecode");
    let mut temp_file = OpenOptions::new().read(true).append(true).create(true).open(&temp_path)
        .context(TransformationError::FailedToCreateImportDestFile)?;
    // Write CBOR header
    temp_file.write_all(cbor_string_header.as_ref())
        .context(TransformationError::FailedToCreateImportDestFile)?;
    // Write all bytecode
    io::copy(&mut original_file, &mut temp_file)
        .context(TransformationError::FailedToCreateImportDestFile)?;
    temp_file.seek(SeekFrom::Start(0))
        .context(TransformationError::FailedToCreateImportDestFile)?;
    // Compute digest and CID of the new file
    let mut hasher = blake3::Hasher::new();
    io::copy(&mut temp_file, &mut hasher)
        .context(TransformationError::FailedToCreateImportDestFile)?;
    let hash = hasher.finalize();
    const CBOR_CODE: u64 = 0x51;
    let multihash = blake3_hash_to_multihash(*hash.as_bytes()).unwrap();
    let cid = Cid::new_v1(CBOR_CODE, multihash);
    // Move temporary file to its final destination
    let final_path = repo_storage.root.join(cid_to_object_path(&cid));
    if let Some(parent_dir) = final_path.parent() {
        fs::create_dir_all(parent_dir)?
    };
    fs::rename(temp_path, final_path)
        .context(TransformationError::FailedTMoveImportFinalFile)?;
    // Print object cid
    println!("{}", &cid.to_string());
    // return
    Ok(())
}