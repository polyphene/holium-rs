use anyhow::Error as AnyhowError;
use anyhow::Result;
use anyhow::Context;
use std::io::{Cursor, Read, Seek};
use std::convert::{TryInto, TryFrom};
use sk_cbor::Value;
use crate::utils::interplanetary::fs::traits::as_ip_block::{AsInterplanetaryBlock};
use crate::utils::local::context::LocalContext;
use cid::Cid;
use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;

pub struct ModuleBytecode {
    pub bytecode: Cursor<Vec<u8>>
}

impl ModuleBytecode {
    pub fn new(bytecode: Vec<u8>) -> Self {
        ModuleBytecode{ bytecode: Cursor::new(bytecode) }
    }
}

impl AsInterplanetaryBlock<Cursor<Vec<u8>>> for ModuleBytecode {
    fn codec() -> BlockMulticodec {
        BlockMulticodec::Raw
    }

    fn get_content(&self) -> Cursor<Vec<u8>> {
        self.bytecode.clone()
    }

    fn from_content(content: &Cursor<Vec<u8>>) -> Result<Box<Self>> {
        Ok(Box::new(ModuleBytecode{bytecode: content.clone() }))
    }
}
