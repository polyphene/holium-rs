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

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate scalar data kind")]
    FailedToManipulate,
}

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
        Ok(Box::new(ScalarData{content: content.get_ref().clone() }))
    }
}