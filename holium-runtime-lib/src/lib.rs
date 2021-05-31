pub(crate) mod env;
pub mod error;

use crate::env::HoliumEnv;
use crate::error::HoliumRuntimeError;
use wasmer::{ImportObject, Instance, Module, Store, Val};

/*****************************************
 * Library
 *****************************************/

pub struct HoliumRuntime {
    #[allow(dead_code)]
    imports: ImportObject,
    runtime: Instance,
}

impl HoliumRuntime {
    #[allow(dead_code)]
    pub fn new(wasm: &[u8]) -> Result<HoliumRuntime, HoliumRuntimeError> {
        let store = Store::default();
        let module = Module::new(&store, wasm)?;

        let holium_env: HoliumEnv = env::HoliumEnv::new();
        let imports: ImportObject = holium_env.import_object(&module);

        let runtime = Instance::new(&module, &imports)?;

        Ok(HoliumRuntime { imports, runtime })
    }

    pub fn run(&self, arguments: &[Val]) -> Result<Box<[Val]>, HoliumRuntimeError> {
        let main = self.runtime.exports.get_function("main")?;
        let result = main.call(arguments)?;

        Ok(result)
    }
}
