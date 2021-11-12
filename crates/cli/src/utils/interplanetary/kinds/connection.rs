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
use sk_cbor::cbor_array;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate connection kind")]
    FailedToManipulate,
}

static DISCRIMINANT_KEY_V0: &str = "cx_0";

pub struct Connection {
    pub tail_selector: Cid,
    pub head_selector: Cid,
}

impl Connection {
    pub fn new(tail_selector: Cid, head_selector: Cid) -> Self {
        Connection { tail_selector, head_selector }
    }
}

impl From<Connection> for sk_cbor::Value {
    fn from(object: Connection) -> Self {
        let tail_selector_link: Value = Link(object.tail_selector).into();
        let head_selector_link: Value = Link(object.head_selector).into();
        let content = cbor_array![
            tail_selector_link,
            head_selector_link,
        ];
        cbor_map! {
            "typedVersion" => DISCRIMINANT_KEY_V0,
            "content" => content,
        }
    }
}

impl TryFrom<sk_cbor::Value> for Connection {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        if let Value::Map(map) = value {
            if map.get(0).is_some() {
                let (k, content) = &map[0];
                if let Value::Array(tuple) = content {
                    if tuple.get(0..2).is_some() {
                        let Link(tail_selector) = Link::try_from(tuple[0].clone())?;
                        let Link(head_selector) = Link::try_from(tuple[1].clone())?;
                        return Ok(Connection{ tail_selector, head_selector })
                    }
                }
            }
        }
        Err(Error::FailedToManipulate.into())
    }
}