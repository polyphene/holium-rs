extern crate wasmer_runtime;

use wasmer_runtime::{Array, Ctx, func, ImportObject, imports, Instance, instantiate, Value, WasmPtr};
use wasmer_runtime::error::{CallError, CacheError};
use wasmer_runtime::error::Error as WasmerRuntimeError;
use std::fmt;

/*****************************************
 * Errors
 *****************************************/
#[derive(Debug)]
pub enum Error {
    WasmerCallError(CallError),
    WasmerCacheError(CacheError),
    WasmerRuntimeError(WasmerRuntimeError),
}

impl From<CallError> for Error {
    fn from(error: CallError) -> Error {
        Error::WasmerCallError(error)
    }
}

impl From<CacheError> for Error {
    fn from(error: CacheError) -> Error {
        Error::WasmerCacheError(error)
    }
}

impl From<WasmerRuntimeError> for Error {
    fn from(error: WasmerRuntimeError) -> Error {
        Error::WasmerRuntimeError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WasmerCallError(inner) => write!(f, "{}", inner),
            Error::WasmerCacheError(inner) => write!(f, "{:?}", inner),
            Error::WasmerRuntimeError(inner) => write!(f, "{}", inner)
        }
    }
}

impl std::error::Error for Error {}

/*****************************************
 * Library
 *****************************************/

pub struct HoliumRuntimeConfig {
    #[allow(dead_code)]
    logging: bool
}

impl HoliumRuntimeConfig {
    #[allow(dead_code)]
    pub fn new(logging: bool) -> HoliumRuntimeConfig {
        HoliumRuntimeConfig {
            logging
        }
    }
}


pub struct HoliumRuntime {
    #[allow(dead_code)]
    imports: ImportObject,
    runtime: Instance,
}


impl HoliumRuntime {
    #[allow(dead_code)]
    pub fn new(wasm: &[u8], config: HoliumRuntimeConfig) -> Result<HoliumRuntime, Error> {
        let imports: ImportObject = generate_imports(config);

        let runtime = instantiate(wasm, &imports)?;

        Ok(HoliumRuntime {
            imports,
            runtime,
        })
    }

    pub fn run(&self, arguments: &[Value]) -> Result<Vec<Value>, Error> {
        let result = self.runtime.call("main", arguments)?;

        Ok(result)
    }
}

fn generate_imports(config: HoliumRuntimeConfig) -> ImportObject {
    let mut imports = imports! {};
    if config.logging {
        imports = imports! {
            // Define the "env" namespace
            "env" => {
                // name
                "print_str" => func!(print_str),
            },
        };
    }

    return imports
}

// print_str is used to log transformations
fn print_str(ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) {
    // Get a slice that maps to the memory currently used by the webassembly
    // instance.
    //
    // Webassembly only supports a single memory for now,
    // but in the near future, it'll support multiple.
    //
    // Therefore, we don't assume you always just want to access first
    // memory and force you to specify the first memory.
    let memory = ctx.memory(0);

    // Use helper method on `WasmPtr` to read a utf8 string
    let string = ptr.get_utf8_string(memory, len).unwrap();

    // Print it!
    println!("Transformation log: {}", string);
}