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

static DISCRIMINANT_KEY_V0: &str = "dt_0";

pub struct DryTransformation<'a> {
    module_bytecode_envelope_cid: &'a Cid,
    handle: &'a str,
}

impl<'a> DryTransformation<'a> {
    pub fn new(module_bytecode_envelope_cid: &'a Cid, handle: &'a str) -> Self {
        DryTransformation{module_bytecode_envelope_cid, handle}
    }
}

impl From<DryTransformation<'_>> for sk_cbor::Value {
    fn from(o: DryTransformation<'_>) -> Self {
        let bytecode_link: Value = Link(o.module_bytecode_envelope_cid).into();
        cbor_map! {
            "typedVersion" => DISCRIMINANT_KEY_V0,
            "bytecode" => bytecode_link,
            "handle" => o.handle,
        }
    }
}