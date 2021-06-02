use serde_json::{json, Map, Value};
use std::cell::Cell;
use std::ops::Deref;
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
/// In our case the data that we want accessible are located both in `tmp_input` & `tmp_output`. In our
/// case the first is read only and the other write only.
#[derive(WasmerEnv, Clone)]
pub struct HoliumEnv {
    #[wasmer(export)]
    memory: LazyInit<Memory>,
    // TODO @PhilippeMts we need to see if a serde_json::Map is our best option
    pub tmp_input: HoliumTmpStorage,
    tmp_output: HoliumTmpStorage,
}

impl HoliumEnv {
    /// Generate a new HoliumEnv
    pub fn new() -> Self {
        Self {
            memory: LazyInit::new(),
            tmp_input: HoliumTmpStorage::new(),
            tmp_output: HoliumTmpStorage::new(),
        }
    }

    /***********************************************************************
     * Temporary storage utils
     ***********************************************************************/

    /// Clears temporary storage for guest's inputs & outputs
    #[allow(dead_code)]
    pub fn clear_tmp(&mut self) {
        self.tmp_input = HoliumTmpStorage::new();
        self.tmp_output = HoliumTmpStorage::new();
    }

    /// Set inputs for guest's module to access
    pub fn set_inputs(&mut self, inputs_map: Map<String, Value>) {
        self.tmp_input = HoliumTmpStorage::from(inputs_map);
    }

    /// Get outputs written by guest's module
    pub fn get_outputs(&mut self) -> Map<String, Value> {
        self.tmp_output.data()
    }

    /***********************************************************************
     * Memory manipulation utils
     ***********************************************************************/

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

    /// Write `&[u8]` in guest `wasmer::Memory`, at a given `wasmer::WasmPtr`.
    ///
    /// In case of problem with memory allocation will return an `ExecutionError`
    fn write_in_memory(
        &self,
        target_value_ptr: WasmPtr<u8, Array>,
        value_slice: &[u8],
    ) -> Result<(), ExecutionError> {
        let memory = self.memory();

        // Allocate necessary memory space on guest
        let guest_value_slice: &[Cell<u8>] =
            match target_value_ptr.deref(memory, 0, value_slice.len() as u32) {
                Some(slice) => slice,
                None => &[],
            };
        if guest_value_slice.len() == 0 {
            return Err(ExecutionError::OutOfMemoryError);
        }

        // Copy bytes to guest
        for i in 0..value_slice.len() {
            guest_value_slice[i].set(value_slice[i]);
        }

        Ok(())
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

    /***********************************************************************
     * Utils
     ***********************************************************************/

    /// Get an import object
    pub fn import_object(&self, module: &Module) -> ImportObject {
        generate_import_object_from_env(module.store(), self.clone())
    }
}

/// HoliumTmpStorage is a structure to help us handle temporary storage that the guest wasm module can
/// write on and read from.
#[derive(Clone, Debug)]
pub struct HoliumTmpStorage {
    store: Arc<Mutex<Map<String, Value>>>,
}

impl From<Map<String, Value>> for HoliumTmpStorage {
    fn from(inputs_map: Map<String, Value>) -> Self {
        HoliumTmpStorage {
            store: Arc::new(Mutex::new(inputs_map)),
        }
    }
}

impl HoliumTmpStorage {
    fn new() -> Self {
        HoliumTmpStorage {
            store: Arc::new(Mutex::new(Map::new())),
        }
    }

    /// Sets a value in the store
    fn set_value(&self, storage_key: String, value: &[u8]) -> Result<(), ExecutionError> {
        let tmp_storage = Arc::clone(&self.store);
        let mut storage: MutexGuard<Map<String, Value>> = match tmp_storage.lock() {
            Ok(storage) => storage,
            Err(poisoned) => poisoned.into_inner(),
        };
        let json_value = json!(value);

        storage.insert(storage_key, json_value);
        Ok(())
    }

    /// Gets a value in the store
    fn get_value(&self, storage_key: String) -> Option<Vec<u8>> {
        let tmp_storage = Arc::clone(&self.store);
        let storage: MutexGuard<Map<String, Value>> = match tmp_storage.lock() {
            Ok(storage) => storage,
            Err(poisoned) => poisoned.into_inner(),
        };
        let serialized_value_clone = storage.get(&storage_key)?.clone();
        let value: Option<Vec<u8>> = match serde_json::from_value(serialized_value_clone) {
            Ok(value) => Some(value),
            Err(_) => None,
        };
        return value;
    }

    /// Retrieve outputs written by guest module
    fn data(&self) -> Map<String, Value> {
        let tmp_storage = Arc::clone(&self.store);
        let storage: MutexGuard<Map<String, Value>> = match tmp_storage.lock() {
            Ok(storage) => storage,
            Err(poisoned) => poisoned.into_inner(),
        };

        return storage.deref().clone();
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

    match env
        .tmp_output
        .set_value(String::from(storage_key), value_slice)
    {
        Ok(_) => 0 as u32,
        Err(e) => u32::from(e),
    }
}

/// Function that will be imported by the Wasm Runtime. It handles communication from the Runtime
/// to `HoliumEnv::tpm_storage` to get a value previously stored.
///
/// It takes a `storage_key` to retrieve a `payload` and write it in memory. It is also passing the
/// length of the payload so that the Runtime can read it.
fn get_payload(
    env: &HoliumEnv,
    storage_key_ptr: WasmPtr<u8, Array>,
    storage_key_len: u32,
    payload_ptr: WasmPtr<u8, Array>,
    result_ptr: WasmPtr<u8, Array>,
) -> u32 {
    let storage_key: &str = match env.get_str_from_memory(storage_key_ptr, storage_key_len) {
        Some(storage_key) => storage_key,
        None => "",
    };
    if storage_key.len() == 0 {
        return u32::from(ExecutionError::InvalidStorageKeyError);
    }

    let payload_slice: Vec<u8> = match env.tmp_input.get_value(String::from(storage_key)) {
        Some(value) => value,
        None => b"".to_vec(),
    };

    let res: u32 = match env.write_in_memory(payload_ptr, &payload_slice) {
        Err(e) => u32::from(e),
        _ => 0 as u32,
    };
    if res != 0 {
        return res;
    }

    let res: u32 =
        match env.write_in_memory(result_ptr, &(payload_slice.len() as u32).to_le_bytes()) {
            Err(e) => u32::from(e),
            _ => 0 as u32,
        };

    res
}

/// Function that will generate a `Wasmer::ImportObject` for our custom environment.
pub fn generate_import_object_from_env(store: &Store, env: HoliumEnv) -> ImportObject {
    imports! {
        "env" => {
            "set_payload" => Function::new_native_with_env(store, env.clone(), set_payload),
            "get_payload" => Function::new_native_with_env(store, env.clone(), get_payload),
        }
    }
}
