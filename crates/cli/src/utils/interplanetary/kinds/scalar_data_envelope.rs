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
    #[error("failed to manipulate scalar data envelope kind")]
    FailedToManipulate,
}

static DISCRIMINANT_KEY_V0: &str = "sde_0";
pub struct ScalarDataEnvelope {
    pub scalar_data_cid: Cid,
}

impl From<ScalarDataEnvelope> for sk_cbor::Value {
    fn from(object: ScalarDataEnvelope) -> Self {
        let scalar_data_link: Value = Link(object.scalar_data_cid).into();
        cbor_map! {
            "typedVersion" => DISCRIMINANT_KEY_V0,
            "content" => scalar_data_link,
        }
    }
}

impl TryFrom<sk_cbor::Value> for ScalarDataEnvelope {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        if let sk_cbor::Value::Map(tuples) = value {
            let (_, discriminant_key) = tuples.get(0).ok_or(Error::FailedToManipulate)?;
            if *discriminant_key == Value::TextString(DISCRIMINANT_KEY_V0.to_string()) {
                let (_, scalar_data_cid_value) = tuples.get(1).ok_or(Error::FailedToManipulate)?;
                let Link(scalar_data_cid) = Link::try_from(scalar_data_cid_value.clone())?;
                return Ok(ScalarDataEnvelope{ scalar_data_cid });
            }
        }
        Err(Error::FailedToManipulate.into())
    }
}