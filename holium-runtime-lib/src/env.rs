use serde_json::{json, Map, Value};
use std::cell::Cell;
use std::sync::{Arc, Mutex, MutexGuard};
use thiserror::Error;
use wasmer::{
    imports, Array, Function, ImportObject, LazyInit, Memory, Module, Store, WasmPtr, WasmerEnv,
};

/// ExecutionError represents all the errors that might occur during the run of our guest Wasm module
/// while interacting with the host.
#[derive(Error, Debug)]
enum ExecutionError {
    #[error("Invalid storage key")]
    InvalidStorageKeyError,
    #[error("No content to set")]
    NoContentError,
    #[error("Out of memory")]
    OutOfMemoryError,
    #[error("Serialization error")]
    SerializationError(#[from] serde_json::Error),
}

/// It has to be noted that an imported function in a Wasm module can only return one number value
/// (as of now). So to pass errors we need a conversion to u32.
impl From<ExecutionError> for u32 {
    fn from(e: ExecutionError) -> u32 {
        match e {
            ExecutionError::InvalidStorageKeyError => 1,
            ExecutionError::NoContentError => 2,
            ExecutionError::OutOfMemoryError => 3,
            ExecutionError::SerializationError(_) => 4,
        }
    }
}

/// `HoliumEnv` is a structure representing the environment passed to host functions after
/// instantiation but before execution. It serves to expose data that can only be accessed after
/// instantiation.
///
/// In our case the data that we want accessible is `tmp_storage`. This structure represents a temporary
/// storage with which our runtime can interact with.
#[derive(WasmerEnv, Clone)]
pub struct HoliumEnv {
    #[wasmer(export)]
    memory: LazyInit<Memory>,
    // TODO @PhilippeMts we need to see if a serde_json::Map is our best option
    pub tmp_storage: Arc<Mutex<Map<String, Value>>>,
}

impl HoliumEnv {
    /// Generate a new HoliumEnv
    pub fn new() -> Self {
        Self {
            memory: LazyInit::new(),
            tmp_storage: Arc::new(Mutex::new(Map::new())),
        }
    }

    /// Get an import object
    pub fn import_object(&self, module: &Module) -> ImportObject {
        generate_import_object_from_env(module.store(), self.clone())
    }

    /// Function to access the environment memory
    fn memory(&self) -> &Memory {
        self.memory_ref()
            .expect("Memory should be set on `HoliumEnv` first")
    }

    /// Returns an `Option<&[u8]>` type from a `wasmer::Memory`, given a `wasmer::WasmPtr`.
    ///
    /// Returns `None` in case memory information are erroneous.
    fn read_from_memory(&self, value_ptr: WasmPtr<u8, Array>, value_len: u32) -> Option<&[u8]> {
        let memory = self.memory();

        let memory_size = memory.size().bytes().0;

        if value_ptr.offset() as usize + value_len as usize > memory_size
            || value_ptr.offset() as usize >= memory_size
        {
            return None;
        }
        let ptr = unsafe {
            memory
                .view::<u8>()
                .as_ptr()
                .add(value_ptr.offset() as usize) as *const u8
        };
        unsafe { Some(std::slice::from_raw_parts(ptr, value_len as usize)) }
    }

    /// Returns an `Option<&str>` type from a `wasmer::Memory` given a `wasmer::WasmPtr`.
    ///
    /// Returns `None` in case there was no value at the given pointer.
    fn get_str_from_memory(&self, str_ptr: WasmPtr<u8, Array>, str_len: u32) -> Option<&str> {
        let storage_key_slice: &[u8] = match self.read_from_memory(str_ptr, str_len) {
            Some(storage_key_slice) => storage_key_slice,
            None => &[],
        };

        if storage_key_slice.len() == 0 {
            return None;
        }

        let value = match serde_json::from_slice(storage_key_slice) {
            Ok(v) => v,
            Err(_) => "",
        };

        Some(value)
    }

    /// Sets a value in the `HoliumEnv::tmp_storage` of the environment
    fn set_tmp_value(&self, storage_key: String, value: &[u8]) -> Result<(), ExecutionError> {
        let tmp_storage = Arc::clone(&self.tmp_storage);
        let mut storage: MutexGuard<Map<String, Value>> = match tmp_storage.lock() {
            Ok(storage) => storage,
            Err(poisoned) => poisoned.into_inner(),
        };
        let json_value = json!(value);
        println!("set_tmp_value value: {:?}", value);
        println!("set_tmp_value json_value: {:?}", json_value);

        storage.insert(storage_key, json_value);
        Ok(())
    }
}

/// Function that will be imported by the Wasm Runtime. It handles communication from the Runtime
/// to `HoliumEnv::tpm_storage` to set a new value.
///
/// It takes a `payload` to store in the storage at a given `storage_key`.
fn set_payload(
    env: &HoliumEnv,
    storage_key_ptr: WasmPtr<u8, Array>,
    storage_key_len: u32,
    payload_ptr: WasmPtr<u8, Array>,
    payload_len: u32,
) -> u32 {
    let storage_key: &str = match env.get_str_from_memory(storage_key_ptr, storage_key_len) {
        Some(storage_key) => storage_key,
        None => "",
    };
    if storage_key.len() == 0 {
        return u32::from(ExecutionError::InvalidStorageKeyError);
    }

    let value_slice: &[u8] = match env.read_from_memory(payload_ptr, payload_len) {
        Some(value_slice) => value_slice,
        None => &[],
    };
    if value_slice.len() == 0 {
        return u32::from(ExecutionError::NoContentError);
    }

    match env.set_tmp_value(String::from(storage_key), value_slice) {
        Ok(_) => 0 as u32,
        Err(e) => u32::from(e),
    }
}

/// Function that will generate a `Wasmer::ImportObject` for our custom environment.
pub fn generate_import_object_from_env(store: &Store, env: HoliumEnv) -> ImportObject {
    imports! {
        "env" => {
            "set_payload" => Function::new_native_with_env(store, env.clone(), set_payload),
        }
    }
}
