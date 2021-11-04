use std::io::Cursor;
use cid::Cid;
use std::collections::HashMap;
use anyhow::Error as AnyhowError;
use anyhow::Result;
use std::convert::{TryInto, TryFrom};
use sk_cbor::Value;
use sk_cbor::cbor_map;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::kinds::link::Link;

static DISCRIMINANT_KEY_V0: &str = "mbe_0";

pub struct ModuleBytecodeEnvelope<'a> {
    module_bytecode_cid: &'a Cid,
}

impl<'a> ModuleBytecodeEnvelope<'a> {
    pub fn new(module_bytecode_cid: &'a Cid) -> Self {
        ModuleBytecodeEnvelope{module_bytecode_cid}
    }
}

impl From<ModuleBytecodeEnvelope<'_>> for sk_cbor::Value {
    fn from(o: ModuleBytecodeEnvelope<'_>) -> Self {
        let content: Value = Link(o.module_bytecode_cid).into();
        cbor_map! {
            "typedVersion" => DISCRIMINANT_KEY_V0,
            "content" => content,
        }
    }
}