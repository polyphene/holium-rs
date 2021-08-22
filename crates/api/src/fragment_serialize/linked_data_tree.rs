use std::{fs, io};
use std::convert::TryFrom;
use std::io::{BufReader, Read};

use anyhow::{Context, Result};
use cid::Cid;
use serde_cbor::Value;

use crate::data::linked_data_tree::Value as LinkedDataTreeValue;
use crate::fragment_serialize::{FragmentedDataDeserResult, HoliumDeserializable};
use crate::fragment_serialize::FragmentSerializeError::WrongDeserializer;

impl HoliumDeserializable for LinkedDataTreeValue {
    fn is_of_type<R: Read>(data_reader: &mut R) -> Result<bool> {
        // read data
        let mut data = Vec::new();
        data_reader.read_to_end(&mut data)?;
        // try to deserialize a CBOR value
        let deser_res: serde_cbor::error::Result<Value> = serde_cbor::from_slice(data.as_slice());
        if let Ok(cbor_value) = deser_res {
            match cbor_value {
                // most scalar values are considered valid deserializable objects
                Value::Null | Value::Bool(_) | Value::Integer(_) | Value::Float(_) | Value::Bytes(_) | Value::Text(_) => Ok(true),
                // maps and tags are not deserializable
                Value::Map(_) | Value::Tag(_, _) | Value::__Hidden => Ok(false),
                // the content of arrays should only be Links, containing CIDs, to be proper deserializable objects
                Value::Array(arr_values) => {
                    Ok(
                        arr_values.iter().all(|v| {
                            match v {
                                // Here we do NOT check for a Tag because of serde/tag limitations, but directly for Bytes.
                                // Check that the array only includes proper Links
                                Value::Bytes(bytes) =>
                                    bytes.len() > 1
                                        && bytes.get(..1).unwrap() == b"\x00"
                                        && Cid::try_from(bytes.get(1..).unwrap()).is_ok(),
                                _ => false
                            }
                        })
                    )
                }
            }
        } else {
            Ok(false)
        }
    }

    fn value_from_bytes(data: &[u8]) -> FragmentedDataDeserResult<Box<Self>> {
        // first deserialize CBOR value
        let cbor_value: Value = serde_cbor::from_slice(data).context(WrongDeserializer).unwrap();
        let value = Box::new(LinkedDataTreeValue::from_cbor(Vec::from(data)));
        // then build the links field
        let links: Vec<Cid> = if let Value::Array(arr_values) = cbor_value {
            // in case of a recursive object, parse every links
            let mut cids: Vec<Cid> = Vec::with_capacity(arr_values.len());
            for arr_value in arr_values {
                if let Value::Bytes(bin_cid) = arr_value {
                    cids.push(Cid::try_from(bin_cid.get(1..).unwrap()).context(WrongDeserializer).unwrap());
                } else {
                    panic!("{:?}", WrongDeserializer)
                }
            }
            cids
        } else {
            // in case of a scalar object, return an empty list of links
            Vec::new()
        };
        // return
        FragmentedDataDeserResult { value, links }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod test_is_of_type {
        use super::*;

        #[test]
        fn can_recognize_scalar_holium_data() {
            let data = hex::decode("f6").unwrap();  // \xf6 is CBOR null value
            assert!(LinkedDataTreeValue::is_of_type(&mut data.as_slice()).unwrap())
        }

        #[test]
        fn can_recognize_recursive_holium_data() {
            let data = hex::decode("81d82a58250001511e2061a9bf10f0ffedc7dc77589ae2ab4ca80b006c806e6636e41b60410cd8f0bbc4").unwrap();
            assert!(LinkedDataTreeValue::is_of_type(&mut data.as_slice()).unwrap())
        }

        #[test]
        fn can_recognize_cbor_map() {
            let data = hex::decode("a100f6").unwrap();
            assert!(!LinkedDataTreeValue::is_of_type(&mut data.as_slice()).unwrap())
        }

        #[test]
        fn can_recognize_array_with_empty_bytes() {
            let data = hex::decode("8140").unwrap();
            assert!(!LinkedDataTreeValue::is_of_type(&mut data.as_slice()).unwrap())
        }

        #[test]
        fn can_recognize_array_with_non_ipld_links() {
            let data = hex::decode("8561496453696E676374686564426F647968456C656374726963").unwrap();
            assert!(!LinkedDataTreeValue::is_of_type(&mut data.as_slice()).unwrap())
        }

        #[test]
        fn can_recognize_non_cbor() {
            let data = hex::decode("8561").unwrap();
            assert!(!LinkedDataTreeValue::is_of_type(&mut data.as_slice()).unwrap())
        }
    }

    mod test_value_from_bytes {
        use super::*;

        #[test]
        fn can_parse_scalar_value() {
            let data = hex::decode("f6").unwrap();  // \xf6 is CBOR null value
            let res = LinkedDataTreeValue::value_from_bytes(&data);
            assert!(res.links.len() < 1);
            let expected_value = LinkedDataTreeValue::from_cbor(data.to_vec());
            assert_eq!(*res.value, expected_value);
        }

        #[test]
        fn can_parse_recursive_value() {
            let data = hex::decode("81d82a58250001511e2061a9bf10f0ffedc7dc77589ae2ab4ca80b006c806e6636e41b60410cd8f0bbc4").unwrap();
            let res = LinkedDataTreeValue::value_from_bytes(&data);
            assert_eq!(res.links.len(), 1);
            assert_eq!(res.links[0], Cid::try_from("bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq").unwrap());
            let expected_value = LinkedDataTreeValue::from_cbor(data.to_vec());
            assert_eq!(*res.value, expected_value);
        }

        #[test]
        fn can_parse_recursive_value_with_duplicate_link() {
            let data = hex::decode("82D82A58250001511E2061A9BF10F0FFEDC7DC77589AE2AB4CA80B006C806E6636E41B60410CD8F0BBC4D82A58250001511E2061A9BF10F0FFEDC7DC77589AE2AB4CA80B006C806E6636E41B60410CD8F0BBC4").unwrap();
            let res = LinkedDataTreeValue::value_from_bytes(&data);
            assert_eq!(res.links.len(), 2);
            assert!(res.links.iter().all(|link|
                *link == Cid::try_from("bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq").unwrap()
            ));
            let expected_value = LinkedDataTreeValue::from_cbor(data.to_vec());
            assert_eq!(*res.value, expected_value);
        }

        #[test]
        #[should_panic(expected = "wrong deserializer")]
        fn parsing_non_cbor_value_should_panic() {
            let data = hex::decode("8561").unwrap();
            LinkedDataTreeValue::value_from_bytes(&data);
        }
    }
}