use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;

use anyhow::Result;

use std::io::Cursor;

pub struct ScalarData {
    pub content: Vec<u8>,
}

impl AsInterplanetaryBlock<Cursor<Vec<u8>>> for ScalarData {
    fn codec() -> BlockMulticodec {
        BlockMulticodec::DagCbor
    }

    fn get_content(&self) -> Cursor<Vec<u8>> {
        Cursor::new(self.content.clone())
    }

    fn from_content(content: &Cursor<Vec<u8>>) -> Result<Box<Self>> {
        Ok(Box::new(ScalarData {
            content: content.get_ref().clone(),
        }))
    }
}
