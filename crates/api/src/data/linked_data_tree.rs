//! Linked data trees recursively hold dag-cbor representation of holium data on IPFS / IPLD.

use anyhow::{anyhow, Error, Result};
use cid::Cid;
use cid::multihash::{Code, MultihashDigest};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use serde_cbor::to_vec;
use serde_cbor::Value as CborValue;

use crate::data::data_tree;

const HASHING_ALGO: Code = Code::Blake3_256;
const IPLD_CBOR_TAG: u64 = 42;

/// Nodes all store their IPLD CBOR representation and related CID.
#[derive(Debug, PartialEq)]
pub struct Value {
    pub cbor: Vec<u8>,
    pub cid: Cid,
}

impl Value {
    pub(crate) fn from_cbor(cbor: Vec<u8>) -> Self {
        // compute CID
        const DAG_CBOR_CODE: u64 = 0x71;
        let digest = HASHING_ALGO.digest(&cbor);
        let cid = Cid::new_v1(DAG_CBOR_CODE, digest);
        // return
        Value { cbor, cid }
    }

    fn from_children_cids(children_cids: Vec<Cid>) -> Result<Self> {
        // create CBOR array value
        let cbor_array = CborValue::Array(
            children_cids
                .into_iter()
                .map(|cid| {
                    // get bin digest
                    let digest = cid.to_bytes();
                    // prefix the digest with multibase identity code
                    const MULTIBASE_IDENTITY_PREFIX: &[u8] = b"\x00";
                    let multibase_digest = [MULTIBASE_IDENTITY_PREFIX, digest.as_ref()].concat();
                    // create tagged CBOR byte string
                    let mut cbor_val: CborValue = CborValue::Bytes(multibase_digest);
                    cbor_val = CborValue::Tag(IPLD_CBOR_TAG, Box::from(cbor_val));
                    cbor_val
                })
                .collect()
        );
        // Serialize IPLD CBOR value
        let cbor = ser_ipld_cbor(&cbor_array)?;
        // compute CID and return
        Ok(Self::from_cbor(cbor))
    }
}

/// Serializes an IPLD object in respect to DAG-CBOR specifications.
fn ser_ipld_cbor(val: &CborValue) -> Result<Vec<u8>> {
    // First serialize the CBOR value. In principle, this should be enough. Unfortunately it is not
    // as, because of a limitation of serde, tags do not appear after serialization.
    let bin: Vec<u8> = to_vec(val)?;
    // Thus, we have here to manually include tags related to IPLD Links in the serialized object.
    Ok(replace_cids_with_links(&bin))
}

/// Find CIDs in a CBOR serialized object and replace them with related Links, that is prefix them
/// with CBOR Tags.
fn replace_cids_with_links(before: &[u8]) -> Vec<u8> {
    lazy_static! {
        // The regular expression is responsible for spotting CIDs byte representations.
        // \xd8\x2a : tag(42)
        // \x58\x25 : bytes(37)
        // \x00 : multibase identity code
        // \x01 : CID version code
        // \x51 (CBOR) or \x71 (MerkleDAG CBOR) : multicodec content type
        // \x1e\x20.{32} : multihash content address
        static ref RE: Regex = Regex::new(r"(?-u)(?:\xd8\x2a)?(?P<cid_bytes>\x58\x25\x00\x01(?:\x51|\x71)\x1e\x20.{32})").unwrap();
    }
    // prefix the CID with a CBOR Tag
    let replacement_str = &b"\xd8\x2a$cid_bytes"[..];
    Vec::from(
        RE.replace_all(before, replacement_str)
    )
}

/// Nodes all hold their inner value, made of a CBOR IPLD representation and own CID, and also point
/// to their children.
#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: Value,
    pub children: Vec<Node>,
}

impl Node {
    /// Create linked data leaf node from a scalar value
    fn from_scalar_value(v: data_tree::Value) -> Result<Self> {
        // compute serialized CBOR binary representation
        let cbor = v.to_cbor_vec()?;
        // compute CID
        const CBOR_CODE: u64 = 0x51;
        let digest = HASHING_ALGO.digest(&cbor);
        let cid = Cid::new_v1(CBOR_CODE, digest);
        // return
        Ok(Node { value: Value { cbor, cid }, children: vec![] })
    }

    /// Create the linked data representation of any holium data
    pub fn from_data_tree(n: data_tree::Node) -> Result<Self> {
        if let Some(v) = n.value {
            Node::from_scalar_value(v)
        } else {
            let (children, failures): (Vec<Node>, Vec<Error>) = n.children
                .into_iter()
                .map(|c| Node::from_data_tree(c))
                .partition_result();
            if !failures.is_empty() {
                return Err(anyhow!("failed to create linked data tree"));
            }
            let value = Value::from_children_cids(
                children.iter().map(|c| c.value.cid).collect()
            )?;
            Ok(Node { value, children })
        }
    }
}


#[cfg(test)]
mod tests {
    use cid::Version;

    use super::*;

    mod test_replace_cids_with_links {
        use super::*;

        const TEST_HASH: &str = "fa60a1bf690cbfdde76c4847daf15ee398ba1c81d17e6a3a24c2535b6df46c7f";
        const CID_PREFIX: &str = "0001711e20";
        const BYTES_37_PREFIX: &str = "5825";     // CBOR prefix for 37-byte byte strings
        const LINK_PREFIX: &str = "d82a";

        #[test]
        fn can_replace_cids_with_links() {
            let cid_str = format!("{}{}{}", BYTES_37_PREFIX, CID_PREFIX, TEST_HASH);
            let cid = hex::decode(&cid_str).unwrap();
            let link = replace_cids_with_links(cid.as_ref());
            assert_eq!(
                hex::encode(link),
                format!("{}{}", LINK_PREFIX, &cid_str)
            )
        }

        #[test]
        fn can_replace_cids_twice_safely() {
            let cid_with_link_str = format!("{}{}{}{}", LINK_PREFIX, BYTES_37_PREFIX, CID_PREFIX, TEST_HASH);
            let cid_with_link = hex::decode(&cid_with_link_str).unwrap();
            let _link = replace_cids_with_links(cid_with_link.as_ref());
            assert_eq!(
                hex::encode(cid_with_link),
                cid_with_link_str
            )
        }

        #[test]
        fn can_check_cid_before_replacing_with_link() {
            // we here create a wrong CID by shortening the hash
            let (wrong_hash_str,_) = TEST_HASH.split_at(TEST_HASH.len() - 2);
            let wrong_cid_str = format!("{}{}{}", BYTES_37_PREFIX, CID_PREFIX, wrong_hash_str);
            let wrong_cid = hex::decode(&wrong_cid_str).unwrap();
            let link = replace_cids_with_links(wrong_cid.as_ref());
            assert_eq!(
                hex::encode(link),
                wrong_cid_str
            )
        }
    }

    #[test]
    fn can_compute_cid_of_null_value() {
        // compute cid
        let data_tree = data_tree::Node::new(CborValue::Null);
        let linked_data_tree = Node::from_data_tree(data_tree).unwrap();
        let cid = linked_data_tree.value.cid;
        let multihash = cid.hash();
        // test individual CID components
        assert_eq!(
            cid.version(),
            Version::V1
        );
        assert_eq!(
            cid.codec(),
            0x51
        );
        assert_eq!(
            multihash.code(),
            0x1e
        );
        assert_eq!(
            multihash.size(),
            32
        );
        assert_eq!(
            multihash.digest(),
            hex::decode("61a9bf10f0ffedc7dc77589ae2ab4ca80b006c806e6636e41b60410cd8f0bbc4").unwrap()
        );
        // test CID v1 string format
        assert_eq!(
            cid.to_string(),
            "bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq"
        )
    }

    #[test]
    fn can_compute_cid_of_recursive_value() {
        // compute cid
        let data_tree = data_tree::Node::new(CborValue::Array(vec![CborValue::Null]));
        let linked_data_tree = Node::from_data_tree(data_tree).unwrap();
        // test child CID string
        assert_eq!(
            linked_data_tree.children.len(),
            1
        );
        assert_eq!(
            linked_data_tree.children[0].value.cid.to_string(),
            "bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq"
        );

        // Test parent CID string
        //
        // Child's CID :
        // multibase-prefix :           0x00
        // cid-version :                0x01
        // multicodec-content-type :    0x51
        // multihash-content-address :  0x1e - 0x20 - 0x61a9bf10f0ffedc7dc77589ae2ab4ca80b006c806e6636e41b60410cd8f0bbc4
        //
        // Link : encode the child's CID as a CBOR byte-string (major type 2), and associate
        // it with CBOR tag 42.

        // NB : serde_cbor v0.11.1 suffers from the current limitation :
        // > Tags are ignored during deserialization and can't be emitted during serialization.

        assert_eq!(
            linked_data_tree.value.cbor,
            hex::decode("81d82a58250001511e2061a9bf10f0ffedc7dc77589ae2ab4ca80b006c806e6636e41b60410cd8f0bbc4").unwrap()
        );
        assert_eq!(
            linked_data_tree.value.cid.to_string(),
            "bafyr4iaboxtdci2fq5i65vzoe4jzjeqsafdmh46mz6qzhyfszd4ocealaa"
        );
    }
}