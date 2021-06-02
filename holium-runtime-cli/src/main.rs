extern crate holium_runtime_lib;

use clap::{App, Arg, Values};
use holium_runtime_lib::error::HoliumRuntimeError;
use holium_runtime_lib::*;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), HoliumRuntimeError> {
    let matches = App::new("Holium Runtime")
        .version("1.0")
        .author("Polyphene <contact@polyphene.io>")
        .about("Runtime CLI for transformations")
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("inputs")
                .short('i')
                .multiple(true)
                .takes_value(true)
                .about("Specify the inputs that should be used to run the transformation"),
        )
        .get_matches();

    let file_path: &str = match matches.value_of("INPUT") {
        Some(x) => x,
        None => panic!("Input should be relative path to wasm file"),
    };

    let formatted_inputs: Map<String, Value> = match matches.values_of("inputs") {
        Some(inputs) => format_inputs(inputs),
        None => Map::new(),
    };

    let wasm = fetch_wasm(file_path);

    let mut runtime: HoliumRuntime = HoliumRuntime::new()?;
    runtime.instantiate(&wasm, formatted_inputs)?;
    let results = runtime.run()?;

    println!("------------------- RESULTS -------------------");
    for (key, value) in results {
        let value: Vec<u8> = match serde_json::from_value(value) {
            Ok(value) => value,
            Err(_) => vec![],
        };
        println!("KEY : {:}", key);
        println!("VALUE : {:}", std::str::from_utf8(&value).unwrap());
        println!();
    }

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
        Err(error) => panic!(
            "Problem getting transformation {}: {:?}",
            relative_path, error
        ),
        Ok(file) => file,
    };

    file_as_bytes
}

fn format_inputs(inputs: Values) -> Map<String, Value> {
    let mut formatted_inputs: Map<String, Value> = Map::new();

    for input in inputs {
        // Retrieve key & value from input
        let (key, value) = match input.split_once('=') {
            Some(parts) => parts,
            None => panic!("Input is not properly formatted."),
        };

        // Get value as bytes
        let mut value_bytes: Vec<u8> = value.as_bytes().to_vec();

        // Clean up backslash
        value_bytes.retain(|&b| b != 92);

        // Format value and insert in map
        let json_value: Value = json!(value_bytes);
        formatted_inputs.insert(String::from(key), json_value);
    }

    return formatted_inputs;
}
