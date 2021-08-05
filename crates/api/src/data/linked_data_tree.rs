//! Linked data trees recursively hold dag-cbor representation of holium data on IPFS / IPLD.

use anyhow::{anyhow, Error, Result};
use cid::Cid;
use cid::multihash::{Code, MultihashDigest};
use itertools::Itertools;
use serde_cbor::to_vec;
use serde_cbor::Value as CborValue;

use crate::data::data_tree;

const HASHING_ALGO: Code = Code::Blake3_256;

/// Nodes all store their IPLD CBOR representation and related CID.
struct Value {
    cbor: Vec<u8>,
    cid: Cid,
}

impl Value {
    fn from_children_cids(children_cids: Vec<Cid>) -> Result<Self> {
        // compute serialized CBOR binary representation
        let cbor_value = CborValue::Array(
            children_cids
                .into_iter()
                .map(|cid| {
                    let digest = cid.to_bytes();
                    const MULTIBASE_IDENTITY_PREFIX: &[u8] = b"\x00";
                    let multibase_digest = [MULTIBASE_IDENTITY_PREFIX, digest.as_ref()].concat();
                    let cbor_byte_string = CborValue::Bytes(multibase_digest);
                    let tagged_value = CborValue::Tag(42, Box::from(cbor_byte_string));
                    return tagged_value;
                })
                .collect()
        );
        let cbor = to_vec(&cbor_value)?;
        // compute CID
        const DAG_CBOR_CODE: u64 = 0x71;
        let digest = HASHING_ALGO.digest(&cbor);
        let cid = Cid::new_v1(DAG_CBOR_CODE, digest);
        // return
        Ok(Value { cbor, cid })
    }
}

/// Nodes all hold their inner value, made of a CBOR IPLD representation and own CID, and also point
/// to their children.
struct Node {
    value: Value,
    children: Vec<Node>,
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
    pub(crate) fn from_data_tree(n: data_tree::Node) -> Result<Self> {
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
            hex::decode("D82A58250001511E2061A9BF10F0FFEDC7DC77589AE2AB4CA80B006C806E6636E41B60410CD8F0BBC4").unwrap()
        );
        assert_eq!(
            linked_data_tree.value.cid.to_string(),
            "bafyr4ih2mcq362imx7o6o3cii7npcxxdtc5bzaorpzvdujgcknnw35dmp4"
        );
    }
}