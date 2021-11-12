//! Data trees are responsible for recursively holding holium data. Leaves hold scalar CBOR values
//! while non-leaf nodes point to ordered children.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_cbor::to_vec;
use serde_cbor::Value as CborValue;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// Value held by the leaf of a data tree
pub(crate) enum Value {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    Bytes(Vec<u8>),
    Text(String),
}

impl Value {
    pub(crate) fn to_cbor(&self) -> CborValue {
        match self {
            Value::Null => CborValue::Null,
            Value::Bool(v) => CborValue::Bool(*v),
            Value::Integer(v) => CborValue::Integer(*v),
            Value::Float(v) => CborValue::Float(*v),
            Value::Bytes(v) => CborValue::Bytes(v.clone()),
            Value::Text(v) => CborValue::Text(v.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// Recursive structure building simple data trees
pub struct Node {
    pub(crate) value: Option<Value>,
    pub(crate) children: Vec<Node>,
}

impl Node {
    /// Create a data tree from a Cbor value
    pub fn new(src_value: CborValue) -> Self {
        fn new_leaf(v: Value) -> Node {
            Node {
                value: Some(v),
                children: vec![],
            }
        }
        fn new_non_leaf(children: Vec<Node>) -> Node {
            Node {
                value: None,
                children,
            }
        }

        match src_value {
            CborValue::Null => new_leaf(Value::Null),
            CborValue::Bool(v) => new_leaf(Value::Bool(v)),
            CborValue::Integer(v) => new_leaf(Value::Integer(v)),
            CborValue::Float(v) => new_leaf(Value::Float(v)),
            CborValue::Bytes(v) => new_leaf(Value::Bytes(v)),
            CborValue::Text(v) => new_leaf(Value::Text(v)),
            CborValue::Tag(_, boxed_value) => Self::new(*boxed_value),
            CborValue::Array(values) => {
                new_non_leaf(values.into_iter().map(|v| Self::new(v)).collect())
            }
            CborValue::Map(tree_map) => {
                new_non_leaf(tree_map.into_values().map(|v| Self::new(v)).collect())
            }
            CborValue::__Hidden => unreachable!(),
        }
    }
}

impl From<Node> for CborValue {
    fn from(node: Node) -> Self {
        match &node.value {
            Some(value) => value.to_cbor(),
            None => {
                let mut cbor_values: Vec<CborValue> = Vec::new();
                for node in node.children.into_iter() {
                    cbor_values.push(CborValue::from(node));
                }
                CborValue::Array(cbor_values)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn can_represent_null_value() {
        assert_eq!(
            Node::new(CborValue::Null),
            Node {
                value: Some(Value::Null),
                children: vec![]
            }
        )
    }

    #[test]
    fn can_represent_boolean_value() {
        assert_eq!(
            Node::new(CborValue::from(true)),
            Node {
                value: Some(Value::Bool(true)),
                children: vec![]
            }
        );
        assert_eq!(
            Node::new(CborValue::from(false)),
            Node {
                value: Some(Value::Bool(false)),
                children: vec![]
            }
        )
    }

    #[test]
    fn can_represent_array() {
        assert_eq!(
            Node::new(CborValue::from(vec![
                CborValue::from(vec![CborValue::Null]),
                CborValue::Null,
            ])),
            Node {
                value: None,
                children: vec![
                    Node {
                        value: None,
                        children: vec![Node {
                            value: Some(Value::Null),
                            children: vec![]
                        }],
                    },
                    Node {
                        value: Some(Value::Null),
                        children: vec![]
                    },
                ],
            }
        )
    }

    #[test]
    fn can_represent_map() {
        let mut tree_map = BTreeMap::new();
        tree_map.insert(CborValue::Null, CborValue::Integer(0));
        assert_eq!(
            Node::new(CborValue::from(tree_map)),
            Node {
                value: None,
                children: vec![Node {
                    value: Some(Value::Integer(0)),
                    children: vec![]
                },],
            }
        )
    }

    #[test]
    fn can_import_tagged_value() {
        assert_eq!(
            Node::new(CborValue::Tag(0, Box::from(CborValue::Null),)),
            Node {
                value: Some(Value::Null),
                children: vec![]
            }
        )
    }
}
