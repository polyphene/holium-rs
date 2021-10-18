use anyhow::Result;
use std::borrow::Cow;
use thiserror::Error;
use wasmer::{imports, wat2wasm, Cranelift, Instance, Memory, Module, Store, Universal};

mod memory;

#[derive(Debug, Error)]
/// Errors for the [runtime] module.
enum RuntimeError {
    /// This error is thrown when we are trying to write on a module memory with no space left
    #[error("can not write data on module, out of memory")]
    OutOfMemory,
    /// This error is thrown when data could not be read from wasm memory
    #[error("can not read data from module memory")]
    NoData,
}

/// The [MemoryManipulation] trait is to be implemented on wasm linear memory types to allow read
/// and write on them
trait MemoryManipulation {
    /// [write] will write a u8 slice to the guest module linear memory
    fn write(&self, mem_offset: u32, value_slice: &[u8]) -> Result<()>;
    /// [read] will read a u8 slice from the guest module linear memory
    fn read(&self, mem_offset: u32, value_len: usize) -> Option<&[u8]>;
}

/// [WASM_MEM_ALLOC] is the function name in our wasm module that allows us to allocate some memory
/// to retrieve host data and send guest data
const WASM_MEM_ALLOC: &'static str = "__hbindgen_mem_alloc";
/// [MEMORY] is the name of the wasm linear memory of our guest module
const MEMORY: &'static str = "memory";
/// [RET_SIZE] is the size of a return payload from a Holium generated func. In our case it is the size
/// of [Slice], 8
const RET_SIZE: usize = 8;

/// [Slice] is the structure that Holium generated functions will return
#[derive(Clone, Debug)]
struct Slice {
    ptr: u32,
    len: u32,
}

/// [Runtime] is a structure that contains our wasm runtime and associated functions to run wasm modules
#[derive(Clone, Debug)]
pub struct Runtime {
    pub(crate) instance: Instance,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let store = Store::default();

        // TODO: find better way to init
        let init_wasm_bytes: Cow<[u8]> = wat2wasm(br#"(module)"#)?;

        let module = Module::new(&store, init_wasm_bytes)?;

        let imports = imports! {};

        let instance = Instance::new(&module, &imports)?;

        Ok(Runtime { instance })
    }

    /// [instantiate] will create a new wasm runtime instance that contains a wasm module. The wasm
    /// module will be the target of our [run] function.
    pub fn instantiate(&mut self, wasm_bytecode: &[u8]) -> Result<()> {
        let compiler_config = Cranelift::default();

        // Define the engine that will drive everything.
        //
        // In this case, the engine is `wasmer_engine_universal` which roughly
        // means that the executable code will live in memory.
        let engine = Universal::new(compiler_config).engine();

        // Create a store, that holds the engine.
        let store = Store::new(&engine);

        // Let's compile the Wasm module. It is at this step that the Wasm
        // text is transformed into Wasm bytes (if necessary), and then
        // compiled to executable code by the compiler, which is then
        // stored in memory by the engine.
        let module = Module::new(&store, wasm_bytecode)?;

        // Create an import object. Since our Wasm module didn't declare
        // any imports, it's an empty object.
        let import_object = imports! {};

        // And here we go again. Let's instantiate the Wasm module.
        self.instance = Instance::new(&module, &import_object)?;

        Ok(())
    }

    /// [run] will run a given `func` from the wasm instance while also using `data` as an input payload.
    /// Currently data should be a CBOR serialized [data_tree::Node].
    pub fn run(&mut self, func: &str, data: &[u8]) -> Result<Vec<u8>> {
        // Get module linear memory
        let memory = self.memory()?;

        // Retrieve ptr to pass data
        let target_mem_offset = self.guest_mem_alloc(data.len())?;
        memory.write(target_mem_offset, data)?;

        // Alloc space for return ptr
        let ret_mem_offset = self.guest_mem_alloc(RET_SIZE)?;

        // Get & execute func from wasm
        let wasm_func = self.instance.exports.get_function(func)?;
        wasm_func.call(&[
            wasmer::Value::I32(ret_mem_offset as i32),
            wasmer::Value::I32(target_mem_offset as i32),
            wasmer::Value::I32(data.len() as i32),
        ])?;

        // Read ret ptr from memory
        let wasm_res = memory
            .read(ret_mem_offset, RET_SIZE)
            .ok_or(RuntimeError::NoData)?;

        // Get payload ptr and length
        let payload_info: Slice = unsafe { std::ptr::read(wasm_res.as_ptr() as *const _) };

        // Get payload
        Ok(memory
            .read(payload_info.ptr, payload_info.len as usize)
            .ok_or(RuntimeError::NoData)?
            .to_vec())
    }

    /// [guest_mem_alloc] will allocate some memory space on a wasm linear memory to allow for direct
    /// read and write
    fn guest_mem_alloc(&self, size: usize) -> Result<u32> {
        let mem_alloc = self.instance.exports.get_function(WASM_MEM_ALLOC)?;

        let res_target_ptr = mem_alloc.call(&[wasmer::Value::I32(size as i32)])?.to_vec();

        Ok(res_target_ptr[0].unwrap_i32() as u32)
    }

    /// [memory] retrieves linear memory from a wasm module and returns it as a reference
    fn memory(&self) -> Result<&Memory> {
        Ok(self.instance.exports.get_memory(MEMORY)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wasm_bytes() -> Vec<u8> {
        wat2wasm(
            br#"
            (module
              (type $add_one_t (func (param i32) (result i32)))
              (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
                local.get $value
                i32.const 1
                i32.add)
              (export "add_one" (func $add_one_f))
              (memory $memory (export "memory") 17))
            "#,
        )
        .unwrap()
        .to_vec()
    }

    #[test]
    fn can_instantiate_wasm_module() {
        let mut runtime = Runtime::new().unwrap();

        runtime.instantiate(&wasm_bytes()).unwrap();

        let store = Store::default();

        let module = Module::new(&store, &wasm_bytes()).unwrap();

        assert_eq!(
            runtime.instance.module().serialize().unwrap(),
            module.serialize().unwrap()
        );
    }
}
