use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::kinds::link::Link;
use anyhow::Error as AnyhowError;
use anyhow::Result;
use cid::Cid;
use sk_cbor::cbor_array_vec;
use sk_cbor::cbor_map;
use sk_cbor::Value;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::Cursor;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate recursive data kind")]
    FailedToManipulate,
}

pub struct RecursiveData {
    pub elements_cids: Vec<Cid>,
}

impl From<RecursiveData> for sk_cbor::Value {
    fn from(object: RecursiveData) -> Self {
        let links: Vec<sk_cbor::Value> = object
            .elements_cids
            .into_iter()
            .map(|cid| -> sk_cbor::Value { Link(cid).into() })
            .collect();
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
