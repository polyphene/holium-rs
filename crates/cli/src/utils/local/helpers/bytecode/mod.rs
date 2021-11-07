//! Helper methods related to Wasm bytecode

use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use anyhow::{Context, Result};
use thiserror::Error;

use crate::utils::local::helpers::bytecode::Error::{
    FailedToReadImportFile, MissingWasmMagicNumber,
};

#[derive(Error, Debug)]
/// Errors related to Wasm module bytecode
pub enum Error {
    /// Thrown when failing to open or read a Wasm module file
    #[error("failed to read Wasm bytecode: {0}")]
    FailedToReadImportFile(String),
    /// Thrown when WebAssembly 4-byte magic number could not be found in expected bytecode
    #[error("invalid WebAssembly bytecode (4-byte magic number could not be found)")]
    MissingWasmMagicNumber,
}

/// WebAssembly 4-byte magic number
/// Reference : https://webassembly.github.io/spec/core/bikeshed/#binary-magic
pub const WASM_MAGIC_NUMBER: &[u8; 4] = b"\x00\x61\x73\x6D";

/// Validate that a path points to a valid Wasm module bytecode containing the Wasm magic number and
/// read it
pub fn read_all_wasm_module(path: &PathBuf) -> Result<Vec<u8>> {
    // Open original file in read-only mode
    let mut f =
        File::open(path).context(FailedToReadImportFile(path.to_string_lossy().to_string()))?;
    // Read for first bytes to validate the presence of WebAssembly 4-byte magic number
    let mut header_buffer = [0u8; 4];
    if f.read(&mut header_buffer).context(MissingWasmMagicNumber)? < WASM_MAGIC_NUMBER.len()
        || &header_buffer[..] != WASM_MAGIC_NUMBER
    {
        return Err(MissingWasmMagicNumber.into());
    };
    // read the whole file
    f.seek(SeekFrom::Start(0))
        .context(FailedToReadImportFile(path.to_string_lossy().to_string()))?;
    let metadata =
        fs::metadata(path).context(FailedToReadImportFile(path.to_string_lossy().to_string()))?;
    let mut buffer = Vec::with_capacity(metadata.len() as usize);
    f.read_to_end(&mut buffer)
        .context(FailedToReadImportFile(path.to_string_lossy().to_string()))?;
    Ok(buffer)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    fn wasm_path(wasm_filename: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("utils")
            .join("local")
            .join("helpers")
            .join("bytecode")
            .join("assets")
            .join(wasm_filename)
    }

    #[test]
    fn cannot_get_bytes_on_non_existent_path() {
        // initialize a repository manually
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let non_existent_file_path = &temp_dir.path().join("non_existent_file.wasm");

        let res = read_all_wasm_module(non_existent_file_path);
        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("failed to read Wasm bytecode"));
    }

    #[test]
    fn cannot_get_bytes_on_non_wasm_file() {
        // fetch corrupted wasm module
        let corrupted_wasm_path = wasm_path("import_corrupted.wasm");

        let res = read_all_wasm_module(&corrupted_wasm_path);
        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("invalid WebAssembly bytecode"));
    }

    #[test]
    fn can_get_bytes() {
        // fetch wasm module
        let wasm_path = wasm_path("import.wasm");

        let expected_data = fs::read(&wasm_path).unwrap();

        let wasm_module = read_all_wasm_module(&wasm_path).unwrap();

        assert_eq!(expected_data, wasm_module);
    }
}
