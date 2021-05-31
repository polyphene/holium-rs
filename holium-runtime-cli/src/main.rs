extern crate holium_runtime_lib;

use clap::{Arg, App};
use holium_runtime_lib::*;
use std::path::PathBuf;
use std::fs;
use holium_runtime_lib::error::HoliumRuntimeError;

fn main() -> Result<(), HoliumRuntimeError> {
    let matches = App::new("Holium Runtime")
        .version("1.0")
        .author("Polyphene <contact@polyphene.io>")
        .about("Runtime CLI for transformations")
        .arg(Arg::new("INPUT")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let file_path: &str = match matches.value_of("INPUT") {
        Some(x) => x,
        None => panic!("Input should be relative path to wasm file"),
    };

    let wasm = fetch_wasm(file_path);

    let runtime: HoliumRuntime = HoliumRuntime::new(&wasm)?;

    runtime.run(&[])?;

    Ok(())
}

fn fetch_wasm(relative_path: &str) -> Vec<u8> {
    // Convert relative to absolute path
    let path: PathBuf = PathBuf::from(relative_path);
    let absolute_path = fs::canonicalize(&path);
    let absolute_path = match absolute_path {
        Err(error) => panic!("Problem converting to absolute path: {:?}", error),
        Ok(absolute_path) => absolute_path,
    };

    // Read file content and storing it as wasm variable
    let file_as_bytes = fs::read(&absolute_path);
    let file_as_bytes = match file_as_bytes {
        Err(error) => panic!("Problem getting transformation {}: {:?}", relative_path, error),
        Ok(file) => file,
    };


    file_as_bytes
}

