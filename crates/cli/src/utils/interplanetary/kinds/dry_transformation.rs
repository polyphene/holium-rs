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
    #[error("failed to manipulate dry transformation kind")]
    FailedToManipulate,
}

static DISCRIMINANT_KEY_V0: &str = "dt_0";

pub struct DryTransformation {
    pub module_bytecode_envelope_cid: Cid,
    pub handle: String,
}

impl DryTransformation {
    pub fn new(module_bytecode_envelope_cid: Cid, handle: String) -> Self {
        DryTransformation { module_bytecode_envelope_cid, handle }
    }
}

impl From<DryTransformation> for sk_cbor::Value {
    fn from(object: DryTransformation) -> Self {
        let bytecode_link: Value = Link(object.module_bytecode_envelope_cid).into();
        cbor_map! {
            "typedVersion" => DISCRIMINANT_KEY_V0,
            "bytecode" => bytecode_link,
            "handle" => object.handle,
        }
    }
}

impl TryFrom<sk_cbor::Value> for DryTransformation {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        if let sk_cbor::Value::Map(tuples) = value {
            let (_, bytecode_value) = tuples.get(1).ok_or(Error::FailedToManipulate)?;
            let Link(module_bytecode_envelope_cid) = Link::try_from(bytecode_value.clone())?;
            let (_, handle_value) = tuples.get(0).ok_or(Error::FailedToManipulate)?;
            if let sk_cbor::Value::TextString(handle) = handle_value.clone() {
                return Ok(DryTransformation{ module_bytecode_envelope_cid, handle })
            }
        }
        Err(Error::FailedToManipulate.into())
    }
}