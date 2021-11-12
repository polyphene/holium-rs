use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::kinds::link::Link;
use anyhow::Error as AnyhowError;
use anyhow::Result;
use cid::Cid;
use sk_cbor::cbor_map;
use sk_cbor::Value;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::Cursor;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate module bytecode envelope kind")]
    FailedToManipulate,
}

static DISCRIMINANT_KEY_V0: &str = "mbe_0";

pub struct ModuleBytecodeEnvelope {
    pub module_bytecode_cid: Cid,
}

impl ModuleBytecodeEnvelope {
    pub fn new(module_bytecode_cid: Cid) -> Self {
        ModuleBytecodeEnvelope {
            module_bytecode_cid,
        }
    }
}

impl From<ModuleBytecodeEnvelope> for sk_cbor::Value {
    fn from(object: ModuleBytecodeEnvelope) -> Self {
        let content: Value = Link(object.module_bytecode_cid).into();
        cbor_map! {
            "typedVersion" => DISCRIMINANT_KEY_V0,
            "content" => content,
        }
    }
}

impl TryFrom<sk_cbor::Value> for ModuleBytecodeEnvelope {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        if let sk_cbor::Value::Map(tuples) = value {
            let (_, module_bytecode_cid_value) = tuples.get(0).ok_or(Error::FailedToManipulate)?;
            let Link(module_bytecode_cid) = Link::try_from(module_bytecode_cid_value.clone())?;
            return Ok(ModuleBytecodeEnvelope {
                module_bytecode_cid,
            });
        }
        Err(Error::FailedToManipulate.into())
    }
}
