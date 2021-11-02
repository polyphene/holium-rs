use std::convert::{TryInto, TryFrom};
use crate::utils::local::context::LocalContext;
use cid::Cid;
use anyhow::Result;
use anyhow::Context;
use crate::utils::interplanetary::multiformats::{compute_cid, cid_to_path};
use std::fs;
use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use std::io;
use std::io::{Seek, Read, Cursor};
use crate::utils::interplanetary::context::InterplanetaryContext;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to write block in the interplanetary area: {0}")]
    FailedToWriteBlock(String),
}

pub trait AsInterplanetaryBlock<ContentType: Read + Seek> {
    /// Returns the multicodec used for the interplanetary block.
    fn codec() -> BlockMulticodec;

    /// Access the content of the block.
    fn content(&self) -> ContentType;
    // fn content(&self) -> Cursor<Vec<u8>> { Cursor::new(vec![])}

    /// Read and parse a block in the interplanetary area.
    fn read_from_ip_area(cid: &Cid, ip_context: &InterplanetaryContext) -> Result<Box<Self>> {
        todo!()
    }

    /// Write as a new block in the interplanetary area.
    fn write_to_ip_area(&self, ip_context: &InterplanetaryContext) -> Result<Cid> {
        let mut content: ContentType = self.content();
        // compute cid from reader
        let cid = compute_cid(&mut content, &Self::codec())?;
        // compute related block path
        let path = cid_to_path(&cid, &ip_context)?;
        // write file if it does not already exist
        if !path.exists() {
            // create parent directory if necessary
            let parent_path = &path
                .parent()
                .ok_or(Error::FailedToWriteBlock(cid.into()))?;
            if !parent_path.exists() {
                fs::create_dir(&parent_path)
                    .context(Error::FailedToWriteBlock(cid.into()))?;
            }
            // write file
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .context(Error::FailedToWriteBlock(cid.into()))?;
            io::copy(&mut content, &mut file)
                .context(Error::FailedToWriteBlock(cid.into()))?;
            Seek::rewind(&mut content)
                .context(Error::FailedToWriteBlock(cid.into()))?;
        }
        // return CID
        Ok(cid)
    }
}

impl AsInterplanetaryBlock<Cursor<Vec<u8>>> for sk_cbor::Value {
    fn codec() -> BlockMulticodec {
        BlockMulticodec::DagCbor
    }

    fn content(&self) -> Cursor<Vec<u8>> {
        let cloned_value = self.clone();
        let mut encoded_cbor = Vec::new();
        sk_cbor::writer::write(cloned_value, &mut encoded_cbor);
        Cursor::new(encoded_cbor)
    }
}