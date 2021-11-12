use anyhow::Result;
use anyhow::{Context, Error as AnyhowError};

use serde_json::map::Map;
use serde_json::value::Value as JsonValue;
use serde_json::Number;
use sk_cbor::{cbor_array_vec, cbor_map, cbor_unsigned};

use std::convert::{TryFrom, TryInto};

use std::option::Option::Some;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to parse json selector literal")]
    FailedToParseJsonLiteral,
    #[error("failed to manipulate selector kind")]
    FailedToManipulate,
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
    fn from(object: SelectorEnvelope) -> Self {
        let selector: sk_cbor::Value = object.0.into();
        cbor_map! {
            "selector" => selector
        }
    }
}

impl TryFrom<sk_cbor::Value> for SelectorEnvelope {
    type Error = AnyhowError;
    fn try_from(value: sk_cbor::Value) -> Result<Self> {
        if let sk_cbor::Value::Map(tuples) = value {
            if let Some((_, selector_value)) = tuples.get(0) {
                let selector = Selector::try_from(selector_value.clone())?;
                return Ok(SelectorEnvelope(selector));
            }
        }
        Err(Error::FailedToManipulate.into())
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

    pub fn is_union(&self) -> bool {
        match self {
            Selector::ExploreUnion(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<&str> for Selector {
    type Error = AnyhowError;
    fn try_from(selector_str: &str) -> std::prelude::rust_2015::Result<Self, Self::Error> {
        let v: JsonValue =
            serde_json::from_str(&selector_str).context(Error::FailedToParseJsonLiteral)?;
        Selector::try_from(v)
    }
}

impl From<Selector> for sk_cbor::Value {
    fn from(object: Selector) -> Self {
        let (key, child_selector): (&str, sk_cbor::Value) = match object {
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

impl From<Selector> for JsonValue {
    fn from(object: Selector) -> Self {
        let (key, child_selector): (&str, JsonValue) = match object {
            Selector::Matcher(child) => (".", child.into()),
            Selector::ExploreIndex(child) => ("i", { *child }.into()),
            Selector::ExploreRange(child) => ("r", { *child }.into()),
            Selector::ExploreUnion(child) => ("|", { *child }.into()),
        };
        let mut map = Map::new();
        map.insert(key.to_string(), child_selector);
        JsonValue::Object(map)
    }
}

impl TryFrom<sk_cbor::Value> for Selector {
    type Error = AnyhowError;
    fn try_from(value: sk_cbor::Value) -> Result<Self> {
        if let sk_cbor::Value::Map(tuples) = value {
            if tuples.get(0).is_some() {
                let (k, v) = &tuples[0];
                if let sk_cbor::Value::TextString(k) = k {
                    if k == "." {
                        let child = Matcher::try_from(v.clone())?;
                        return Ok(Selector::Matcher(child));
                    } else if k == "i" {
                        let child = ExploreIndex::try_from(v.clone())?;
                        return Ok(Selector::ExploreIndex(Box::new(child)));
                    } else if k == "r" {
                        let child = ExploreRange::try_from(v.clone())?;
                        return Ok(Selector::ExploreRange(Box::new(child)));
                    } else if k == "|" {
                        let child = ExploreUnion::try_from(v.clone())?;
                        return Ok(Selector::ExploreUnion(Box::new(child)));
                    }
                }
            }
        };
        Err(Error::FailedToManipulate.into())
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
    fn from(object: Matcher) -> Self {
        if let Some(label) = object.label {
            cbor_map! {"label" => label}
        } else {
            cbor_map! {}
        }
    }
}

impl From<Matcher> for JsonValue {
    fn from(object: Matcher) -> Self {
        let mut map = Map::new();
        if let Some(label) = object.label {
            map.insert("label".to_string(), JsonValue::String(label));
        }
        JsonValue::Object(map)
    }
}

impl TryFrom<sk_cbor::Value> for Matcher {
    type Error = AnyhowError;
    fn try_from(value: sk_cbor::Value) -> Result<Self> {
        if let sk_cbor::Value::Map(tuples) = value {
            if let Some((_, label_value)) = tuples.get(0) {
                if let sk_cbor::Value::TextString(label) = label_value {
                    return Ok(Matcher {
                        label: Some(label.clone()),
                    });
                }
            }
            return Ok(Matcher { label: None });
        }
        Err(Error::FailedToManipulate.into())
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
    fn from(object: ExploreIndex) -> Self {
        let selector: sk_cbor::Value = { *object.next }.into();
        cbor_map! {
            "i" => cbor_unsigned!( object.index ),
            ">" => selector,
        }
    }
}

impl From<ExploreIndex> for JsonValue {
    fn from(object: ExploreIndex) -> Self {
        let selector: JsonValue = { *object.next }.into();
        let mut map = Map::new();
        map.insert(
            "i".to_string(),
            JsonValue::Number(Number::from_f64(object.index as f64).unwrap()),
        );
        map.insert(">".to_string(), selector);
        JsonValue::Object(map)
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

impl TryFrom<sk_cbor::Value> for ExploreIndex {
    type Error = AnyhowError;
    fn try_from(value: sk_cbor::Value) -> Result<Self> {
        if let sk_cbor::Value::Map(tuples) = value {
            if let Some((_, index_value)) = tuples.get(1) {
                if let sk_cbor::Value::Unsigned(index) = index_value {
                    // check key is "i"
                    if let Some((_, next_value)) = tuples.get(0) {
                        // check key is ">"
                        return Ok(ExploreIndex {
                            index: *index,
                            next: Box::new(next_value.clone().try_into()?),
                        });
                    }
                }
            }
        };
        Err(Error::FailedToManipulate.into())
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
    fn from(object: ExploreRange) -> Self {
        let selector: sk_cbor::Value = { *object.next }.into();
        cbor_map! {
            "^" => cbor_unsigned!( object.start ),
            "$" => cbor_unsigned!( object.end ),
            ">" => selector,
        }
    }
}

impl From<ExploreRange> for JsonValue {
    fn from(object: ExploreRange) -> Self {
        let selector: JsonValue = { *object.next }.into();
        let mut map = Map::new();
        map.insert(
            "^".to_string(),
            JsonValue::Number(Number::from_f64(object.start as f64).unwrap()),
        );
        map.insert(
            "$".to_string(),
            JsonValue::Number(Number::from_f64(object.end as f64).unwrap()),
        );
        map.insert(">".to_string(), selector);
        JsonValue::Object(map)
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

impl TryFrom<sk_cbor::Value> for ExploreRange {
    type Error = AnyhowError;
    fn try_from(value: sk_cbor::Value) -> Result<Self> {
        if let sk_cbor::Value::Map(tuples) = value {
            if let Some((_, start_value)) = tuples.get(2) {
                if let sk_cbor::Value::Unsigned(start) = start_value {
                    // check key is "^"
                    if let Some((_, end_value)) = tuples.get(0) {
                        if let sk_cbor::Value::Unsigned(end) = end_value {
                            // check key is "$"
                            if let Some((_, next_value)) = tuples.get(1) {
                                // check key is ">"
                                return Ok(ExploreRange {
                                    start: *start,
                                    end: *end,
                                    next: Box::new(next_value.clone().try_into()?),
                                });
                            }
                        }
                    }
                }
            }
        };
        Err(Error::FailedToManipulate.into())
    }
}

/****************
ExploreUnion
****************/
#[derive(Debug)]
pub struct ExploreUnion(pub Vec<Selector>);

impl From<ExploreUnion> for sk_cbor::Value {
    fn from(object: ExploreUnion) -> Self {
        let mut selectors = Vec::with_capacity(object.0.len());
        for s in object.0 {
            let cbor_selector: sk_cbor::Value = s.into();
            selectors.push(cbor_selector);
        }
        cbor_array_vec!(selectors)
    }
}

impl From<ExploreUnion> for JsonValue {
    fn from(object: ExploreUnion) -> Self {
        let mut selectors = Vec::with_capacity(object.0.len());
        for s in object.0 {
            let cbor_selector: JsonValue = s.into();
            selectors.push(cbor_selector);
        }
        JsonValue::Array(selectors)
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

impl TryFrom<sk_cbor::Value> for ExploreUnion {
    type Error = AnyhowError;
    fn try_from(value: sk_cbor::Value) -> Result<Self> {
        if let sk_cbor::Value::Array(vec) = value {
            let selectors_res: Result<Vec<Selector>> =
                vec.iter().map(|v| v.clone().try_into()).collect();
            let selectors = selectors_res?;
            return Ok(ExploreUnion(selectors));
        };
        Err(Error::FailedToManipulate.into())
    }
}
