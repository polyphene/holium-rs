pub(crate) mod env;
pub mod error;

use crate::env::HoliumEnv;
use crate::error::HoliumRuntimeError;
use serde_json::{Map, Value};
use std::borrow::Cow;
use wasmer::{wat2wasm, ImportObject, Instance, Module, Store};

/*****************************************
 * Library
 *****************************************/

pub struct HoliumRuntime {
    env: HoliumEnv,
    #[allow(dead_code)]
    imports: ImportObject,
    instance: Instance,
}

impl HoliumRuntime {
    #[allow(dead_code)]
    pub fn new() -> Result<HoliumRuntime, HoliumRuntimeError> {
        let store = Store::default();

        // TODO: find better way to init
        let init_wasm_bytes: Cow<[u8]> = wat2wasm(br#"(module)"#)?;

        let module = Module::new(&store, init_wasm_bytes)?;

        let holium_env: HoliumEnv = env::HoliumEnv::new();
        let imports: ImportObject = ImportObject::new();

        let instance = Instance::new(&module, &imports)?;

        Ok(HoliumRuntime {
            env: holium_env,
            imports: ImportObject::new(),
            instance,
        })
    }

    pub fn instantiate(
        &mut self,
        wasm: &[u8],
        inputs_map: Map<String, Value>,
    ) -> Result<(), HoliumRuntimeError> {
        let store = Store::default();
        let module = Module::new(&store, wasm)?;

        self.env.set_inputs(inputs_map);
        let imports: ImportObject = self.env.import_object(&module);

        let instantiation_result = Instance::new(&module, &imports);
        if instantiation_result.is_err() {
            return Err(HoliumRuntimeError::from(
                instantiation_result.err().unwrap(),
            ));
        }

        self.instance = instantiation_result.ok().unwrap();

        Ok(())
    }

    pub fn run(&mut self) -> Result<Map<String, Value>, HoliumRuntimeError> {
        // Run the transformation
        let main = self.instance.exports.get_function("main")?;
        main.call(&[])?;

        Ok(self.env.get_outputs())
    }
}
