use std::collections::BTreeMap;

use serde_cbor::Value as CborValue;
use serde_json::Value as JsonValue;

use crate::data::importer::Importable;

impl Importable for JsonValue {
    fn to_cbor(&self) -> CborValue {
        match self {
            JsonValue::Null => CborValue::Null,
            JsonValue::Bool(v) => CborValue::Bool(*v),
            JsonValue::Number(v) if { v.is_i64() } => {
                CborValue::Integer(v.as_i64().unwrap() as i128)
            }
            JsonValue::Number(v) if { v.is_u64() } => {
                CborValue::Integer(v.as_u64().unwrap() as i128)
            }
            JsonValue::Number(v) if { v.is_f64() } => CborValue::Float(v.as_f64().unwrap()),
            JsonValue::String(v) => CborValue::Text(v.to_string()),
            JsonValue::Array(v) => {
                CborValue::Array(v.iter().map(|v| v.to_owned().to_cbor()).collect())
            }
            JsonValue::Object(v) => {
                let mut cbor_map: BTreeMap<CborValue, CborValue> = BTreeMap::new();
                for (key, value) in v {
                    cbor_map.insert(CborValue::Text(key.to_string()), value.to_cbor());
                }
                CborValue::Map(cbor_map)
            }
            JsonValue::Number(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_scalar_conversions {
        use super::*;

        #[test]
        fn can_convert_null_value() {
            assert_eq!(JsonValue::Null.to_cbor(), CborValue::Null)
        }

        #[test]
        fn can_convert_boolean_value() {
            assert_eq!(JsonValue::Bool(true).to_cbor(), CborValue::Bool(true))
        }

        #[test]
        fn can_convert_integer_value() {
            use serde_json::Number;
            assert_eq!(
                JsonValue::Number(Number::from(0)).to_cbor(),
                CborValue::Integer(0)
            )
        }

        #[test]
        fn can_convert_float_value() {
            use serde_json::Number;
            assert_eq!(
                JsonValue::Number(Number::from_f64(0.0).unwrap()).to_cbor(),
                CborValue::Float(0.0)
            )
        }

        #[test]
        fn can_convert_string_value() {
            assert_eq!(
                JsonValue::String(String::from("")).to_cbor(),
                CborValue::Text(String::from(""))
            )
        }
    }

    mod test_rec_conversions {
        use super::*;

        #[test]
        fn can_convert_array_value0() {
            assert_eq!(
                JsonValue::Array(vec![JsonValue::Null]).to_cbor(),
                CborValue::Array(vec![CborValue::Null])
            )
        }

        #[test]
        fn can_convert_array_value1() {
            assert_eq!(
                JsonValue::Array(vec![JsonValue::String(String::from(""))]).to_cbor(),
                CborValue::Array(vec![CborValue::Text(String::from(""))])
            )
        }

        #[test]
        fn can_convert_object_value() {
            assert_eq!(
                JsonValue::Object(
                    [(String::from("k"), JsonValue::Null)]
                        .iter()
                        .cloned()
                        .collect()
                )
                .to_cbor(),
                CborValue::Map(
                    [(CborValue::Text(String::from("k")), CborValue::Null)]
                        .iter()
                        .cloned()
                        .collect()
                )
            )
        }
    }
}
