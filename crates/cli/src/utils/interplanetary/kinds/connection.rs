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

static DISCRIMINANT_KEY_V0: &str = "cx_0";

pub struct Connection<'a> {
    tail_selector: &'a Cid,
    head_selector: &'a Cid,
}

impl<'a> Connection<'a> {
    pub fn new(tail_selector: &'a Cid, head_selector: &'a Cid) -> Self {
        Connection { tail_selector, head_selector }
    }
}

impl From<&Connection<'_>> for sk_cbor::Value {
    fn from(o: &Connection<'_>) -> Self {
        let tail_selector_link: Value = Link(o.tail_selector).into();
        let head_selector_link: Value = Link(o.head_selector).into();
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