use crate::utils::interplanetary::context::InterplanetaryContext;
use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::multiformats::{cid_to_path, compute_cid};

use anyhow::Context;

use anyhow::Result;
use cid::Cid;

use std::fs;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read, Seek, Write};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to write block in the interplanetary area: {0}")]
    FailedToWriteBlock(String),
    #[error("failed to create interplanetary block structure from content")]
    FailedCreationFromContent,
}

/// Trait helping with filesystem operations on InterPlanetary blocks
/// The [ ContentType ] is a type suitable to manipulate the content of the block.
/// It supertraits the following traits:
/// - [ `std::io::Read` ] to read the content and write it in a block.
/// - [ `std::io::Write` ] to read a block and write parts of its content in the rust structure.
/// - [ `std::io::Seek` ] to ease successive read and/or write operations.
/// - [ `core::default::Default` ] to ease the creation of a new structure.
pub trait AsInterplanetaryBlock<ContentType: Read + Write + Seek + Default> {
    /// Returns the multicodec used for the interplanetary block.
    fn codec() -> BlockMulticodec;

    /// Access the content of the block.
    fn get_content(&self) -> ContentType;

    /// Create new object from content.
    fn from_content(content: &ContentType) -> Result<Box<Self>>;

    /// Read and parse a block in the interplanetary area.
    fn read_from_ip_area(cid: &Cid, ip_context: &InterplanetaryContext) -> Result<Box<Self>> {
        // compute block path related to the cid
        let path = cid_to_path(&cid, &ip_context)?;
        // read the whole block
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let mut buf_cursor = Cursor::new(buffer);
        let mut content: ContentType = ContentType::default();
        io::copy(&mut buf_cursor, &mut content)?;
        // returned boxed object
        Self::from_content(&content)
    }

    /// Write as a new block in the interplanetary area.
    fn write_to_ip_area(&self, ip_context: &InterplanetaryContext) -> Result<Cid> {
        let mut content: ContentType = self.get_content();
        // compute cid from reader
        let cid = compute_cid(&mut content, &Self::codec())?;
        // compute related block path
        let path = cid_to_path(&cid, &ip_context)?;
        // write file if it does not already exist
        if !path.exists() {
            // create parent directory if necessary
            let parent_path = &path.parent().ok_or(Error::FailedToWriteBlock(cid.into()))?;
            if !parent_path.exists() {
                fs::create_dir(&parent_path).context(Error::FailedToWriteBlock(cid.into()))?;
            }
            // write file
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .context(Error::FailedToWriteBlock(cid.into()))?;
            io::copy(&mut content, &mut file).context(Error::FailedToWriteBlock(cid.into()))?;
            Seek::rewind(&mut content).context(Error::FailedToWriteBlock(cid.into()))?;
        }
        // return CID
        Ok(cid)
    }
}

impl AsInterplanetaryBlock<Cursor<Vec<u8>>> for sk_cbor::Value {
    fn codec() -> BlockMulticodec {
        BlockMulticodec::DagCbor
    }

    fn get_content(&self) -> Cursor<Vec<u8>> {
        let cloned_value = self.clone();
        let mut encoded_cbor = Vec::new();
        sk_cbor::writer::write(cloned_value, &mut encoded_cbor).unwrap();
        Cursor::new(encoded_cbor)
    }

    fn from_content(content: &Cursor<Vec<u8>>) -> Result<Box<Self>> {
        if let Ok(new_object) = sk_cbor::reader::read(content.get_ref()) {
            Ok(Box::new(new_object))
        } else {
            Err(Error::FailedCreationFromContent.into())
        }
    }
}
