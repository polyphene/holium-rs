use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;

use anyhow::Result;

use std::io::Cursor;

pub struct ModuleBytecode {
    pub bytecode: Cursor<Vec<u8>>,
}

impl ModuleBytecode {
    pub fn new(bytecode: Vec<u8>) -> Self {
        ModuleBytecode {
            bytecode: Cursor::new(bytecode),
        }
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
        Ok(Box::new(ModuleBytecode {
            bytecode: content.clone(),
        }))
    }
}
