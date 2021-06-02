extern crate holium_runtime_lib;

use clap::{App, Arg, ArgMatches, Values};
use holium_runtime_lib::error::HoliumRuntimeError;
use holium_runtime_lib::*;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), HoliumRuntimeError> {
    let cli_matches = App::new("Holium Runtime")
        .version("1.0")
        .author("Polyphene <contact@polyphene.io>")
        .about("Runtime CLI for transformations")
        .subcommand(
            App::new("run")
                .about("Used to run a transformation")
                .arg(
                    Arg::new("WASM_FILE")
                        .about("Sets the wasm file to run")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("inputs")
                        .short('i')
                        .multiple(true)
                        .takes_value(true)
                        .about("Specify the inputs that should be used to run the transformation"),
                ),
        )
        .get_matches();

    match cli_matches.subcommand_name() {
        Some(subcommand) => match subcommand {
            "run" => handle_run_subcommand(cli_matches),
            _ => panic!("The subcommand specified is not handled"),
        },
        None => panic!("holium-runtime-cli should have an appropriate subcommand"),
    }
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

fn handle_run_subcommand(cli_matches: ArgMatches) -> Result<(), HoliumRuntimeError> {
    let run_matches = match cli_matches.subcommand_matches("run") {
        Some(matches) => matches,
        None => panic!("Run subcommand should have at least an wasm file path specified"),
    };

    let file_path: &str = match run_matches.value_of("WASM_FILE") {
        Some(file_path) => file_path,
        None => panic!("Input should be relative path to wasm file"),
    };

    let formatted_inputs: Map<String, Value> = match run_matches.values_of("inputs") {
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
