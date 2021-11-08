use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::kinds::link::Link;
use anyhow::Result;
use anyhow::{Context, Error as AnyhowError};
use cid::Cid;
use serde_json::value::Value as JsonValue;
use sk_cbor::values::IntoCborValue;
use sk_cbor::{cbor_array, cbor_array_vec, cbor_map, cbor_unsigned};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::Cursor;
use std::marker::PhantomData;
use std::ops::Deref;
use std::option::Option::Some;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to parse json selector literal")]
    FailedToParseJsonLiteral,
}

/****************
SelectorEnvelope
****************/

#[derive(Debug)]
pub struct SelectorEnvelope(pub Selector);

impl SelectorEnvelope {
    pub fn new(selector_str: &str) -> Result<Self> {
        let v: JsonValue =
            serde_json::from_str(&selector_str).context(Error::FailedToParseJsonLiteral)?;
        let selector = Selector::try_from(v)?;
        Ok(SelectorEnvelope { 0: selector })
    }
}

impl From<SelectorEnvelope> for sk_cbor::Value {
    fn from(o: SelectorEnvelope) -> Self {
        let selector: sk_cbor::Value = o.0.into();
        cbor_map! {
            "selector" => selector
        }
    }
}

/****************
Selector
****************/

#[derive(Debug)]
pub enum Selector {
    Matcher(Matcher),
    ExploreIndex(Box<ExploreIndex>),
    ExploreRange(Box<ExploreRange>),
    ExploreUnion(Box<ExploreUnion>),
}

impl Selector {
    pub fn is_matcher(&self) -> bool {
        match self {
            Selector::Matcher(_) => true,
            _ => false,
        }
    }
}

impl From<Selector> for sk_cbor::Value {
    fn from(o: Selector) -> Self {
        let (key, child_selector): (&str, sk_cbor::Value) = match o {
            Selector::Matcher(child) => (".", child.into()),
            Selector::ExploreIndex(child) => ("i", { *child }.into()),
            Selector::ExploreRange(child) => ("r", { *child }.into()),
            Selector::ExploreUnion(child) => ("|", { *child }.into()),
        };
        cbor_map! {
            key => child_selector
        }
    }
}

impl TryFrom<JsonValue> for Selector {
    type Error = AnyhowError;
    fn try_from(json_value: JsonValue) -> Result<Self> {
        if let JsonValue::Object(map) = json_value {
            if let Some(child) = map.get(".") {
                return Ok(Selector::Matcher(child.clone().try_into()?));
            } else if let Some(child) = map.get("i") {
                return Ok(Selector::ExploreIndex(Box::new(child.clone().try_into()?)));
            } else if let Some(child) = map.get("r") {
                return Ok(Selector::ExploreRange(Box::new(child.clone().try_into()?)));
            } else if let Some(child) = map.get("|") {
                return Ok(Selector::ExploreUnion(Box::new(child.clone().try_into()?)));
            }
        };
        Err(Error::FailedToParseJsonLiteral.into())
    }
}

/****************
Matcher
****************/

#[derive(Debug)]
pub struct Matcher {
    pub label: Option<String>,
}

impl From<Matcher> for sk_cbor::Value {
    fn from(o: Matcher) -> Self {
        if let Some(label) = o.label {
            cbor_map! {"label" => label}
        } else {
            cbor_map! {}
        }
    }
}

impl TryFrom<JsonValue> for Matcher {
    type Error = AnyhowError;
    fn try_from(json_value: JsonValue) -> Result<Self> {
        if let JsonValue::Object(map) = json_value {
            if let Some(label_value) = map.get("label") {
                if let JsonValue::String(label) = label_value {
                    return Ok(Matcher {
                        label: Some(label.to_string()),
                    });
                }
            }
            return Ok(Matcher { label: None });
        }
        Err(Error::FailedToParseJsonLiteral.into())
    }
}

/****************
ExploreIndex
****************/

#[derive(Debug)]
pub struct ExploreIndex {
    pub index: u64,
    pub next: Box<Selector>,
}

impl From<ExploreIndex> for sk_cbor::Value {
    fn from(o: ExploreIndex) -> Self {
        let selector: sk_cbor::Value = { *o.next }.into();
        cbor_map! {
            "i" => cbor_unsigned!( o.index ),
            ">" => selector,
        }
    }
}

impl TryFrom<JsonValue> for ExploreIndex {
    type Error = AnyhowError;
    fn try_from(json_value: JsonValue) -> Result<Self> {
        if let JsonValue::Object(map) = json_value {
            if let Some(index_value) = map.get("i") {
                if let JsonValue::Number(index) = index_value {
                    if let Some(next_value) = map.get(">") {
                        return Ok(ExploreIndex {
                            index: index.as_u64().ok_or(Error::FailedToParseJsonLiteral)?,
                            next: Box::new(next_value.clone().try_into()?),
                        });
                    }
                }
            }
        };
        Err(Error::FailedToParseJsonLiteral.into())
    }
}

/****************
ExploreRange
****************/

#[derive(Debug)]
pub struct ExploreRange {
    pub start: u64,
    pub end: u64,
    pub next: Box<Selector>,
}

impl From<ExploreRange> for sk_cbor::Value {
    fn from(o: ExploreRange) -> Self {
        let selector: sk_cbor::Value = { *o.next }.into();
        cbor_map! {
            "^" => cbor_unsigned!( o.start ),
            "$" => cbor_unsigned!( o.end ),
            ">" => selector,
        }
    }
}

impl TryFrom<JsonValue> for ExploreRange {
    type Error = AnyhowError;
    fn try_from(json_value: JsonValue) -> Result<Self> {
        if let JsonValue::Object(map) = json_value {
            if let Some(start_value) = map.get("^") {
                if let JsonValue::Number(start) = start_value {
                    if let Some(end_value) = map.get("$") {
                        if let JsonValue::Number(end) = end_value {
                            if let Some(next_value) = map.get(">") {
                                return Ok(ExploreRange {
                                    start: start.as_u64().ok_or(Error::FailedToParseJsonLiteral)?,
                                    end: end.as_u64().ok_or(Error::FailedToParseJsonLiteral)?,
                                    next: Box::new(next_value.clone().try_into()?),
                                });
                            }
                        }
                    }
                }
            }
        };
        Err(Error::FailedToParseJsonLiteral.into())
    }
}

/****************
ExploreUnion
****************/
#[derive(Debug)]
pub struct ExploreUnion(pub Vec<Selector>);

impl From<ExploreUnion> for sk_cbor::Value {
    fn from(o: ExploreUnion) -> Self {
        let mut selectors = Vec::with_capacity(o.0.len());
        for s in o.0 {
            let cbor_selector: sk_cbor::Value = s.into();
            selectors.push(cbor_selector);
        }
        cbor_array_vec!(selectors)
    }
}

impl TryFrom<JsonValue> for ExploreUnion {
    type Error = AnyhowError;
    fn try_from(json_value: JsonValue) -> Result<Self> {
        if let JsonValue::Array(vec) = json_value {
            let selectors_res: Result<Vec<Selector>> =
                vec.iter().map(|v| v.clone().try_into()).collect();
            let selectors = selectors_res?;
            return Ok(ExploreUnion(selectors));
        };
        Err(Error::FailedToParseJsonLiteral.into())
    }
}
