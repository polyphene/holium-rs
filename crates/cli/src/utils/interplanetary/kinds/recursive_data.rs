use std::io::Cursor;
use cid::Cid;
use std::collections::HashMap;
use anyhow::Error as AnyhowError;
use anyhow::Result;
use std::convert::{TryInto, TryFrom};
use sk_cbor::Value;
use sk_cbor::cbor_map;
use sk_cbor::cbor_array_vec;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::kinds::link::Link;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate recursive data kind")]
    FailedToManipulate,
}

pub struct RecursiveData {
    pub elements_cids: Vec<Cid>,
}

impl From<RecursiveData> for sk_cbor::Value {
    fn from(o: RecursiveData) -> Self {
        let links: Vec<sk_cbor::Value> = o.elements_cids.into_iter().map(|cid| -> sk_cbor::Value { Link(cid).into() }).collect();
        cbor_array_vec!(links)
    }
}

impl TryFrom<sk_cbor::Value> for RecursiveData {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        if let Value::Array(links) = value {
            let elements_cids = links
                .into_iter()
                .map(|value| Link::try_from(value.clone()))
                .collect::<Result<Vec<Link>>>()?
                .into_iter()
                .map(|link| link.0)
                .collect();
            Ok(RecursiveData { elements_cids })
        } else {
            Err(Error::FailedToManipulate.into())
        }
    }
}