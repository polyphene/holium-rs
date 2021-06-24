//! The `pack` crate is meant to provide essential structures and functions to manipulate holium
//! data, also called _packs_ because of its connection with [MessagePack](https://msgpack.org/),
//! in the Holium Framework.

pub mod importers;
pub mod store;

use rmp::decode::{read_marker};
use rmp::Marker;
use std::io::Write;

/// The default type to handle data in the Holium framework.
pub enum HoliumPack {
    /// A primitive value. HoliumPack primitive types are a subset of [MessagePack](https://msgpack.org/) types.
    Primitive(Vec<u8>),
    /// All non-primitive values are _arrays_.
    Array(Vec<HoliumPack>),
}

/// Check if a MessagePack vector hold an HoliumPack primitive value.
fn is_primitive_pack(mut buf: &[u8]) -> bool {
    let marker = read_marker(&mut buf);
    return match marker {
        Ok(m) => {
            match m {
                Marker::Null
                | Marker::False
                | Marker::True
                | Marker::U8
                | Marker::U16
                | Marker::U32
                | Marker::U64
                | Marker::I8
                | Marker::I16
                | Marker::I32
                | Marker::I64
                | Marker::F32
                | Marker::F64
                | Marker::Str8
                | Marker::Str16
                | Marker::Str32
                | Marker::Bin8
                | Marker::Bin16
                | Marker::Bin32 => true,
                Marker::FixStr(u8) => u8 < 32,
                Marker::FixPos(u8) => u8 < 128,
                Marker::FixNeg(i8) => -32 <= i8 && i8 < 0,
                _ => false
            }
        }
        Err(_) => { false }
    };
}

impl HoliumPack {
    /// Constructs a new HoliumPack::Primitive.
    ///
    /// # Panics
    ///
    /// Providing a buffer that is not a valid HoliumPack primitive will cause this constructor to panic.
    pub fn new_primitive(buf: Vec<u8>) -> HoliumPack {
        // TODO check type and everything
        if !is_primitive_pack(&buf) { panic!("Expected a valid holium primitive, got {:?}.", buf) }
        HoliumPack::Primitive(buf)
    }

    /// Constructs a new HoliumPack::Array.
    ///
    /// # Panics
    ///
    /// Providing a vector that cannot be represented as a valid HoliumPack will cause this constructor to panic.
    pub fn new_array(vec: Vec<HoliumPack>) -> HoliumPack {
        // TODO check size or panic
        HoliumPack::Array(vec)
    }

    /// Recursively turns a structured HoliumPack into a valid single binarized pack.
    pub fn as_bytes(&self) -> Vec<u8> {
        return match self {
            HoliumPack::Primitive(buf) => buf.clone(),
            HoliumPack::Array(v) => {
                let mut pack: Vec<u8> = Vec::new();
                rmp::encode::write_array_len(&mut pack, v.len() as u32).unwrap();
                for p in v.iter() {
                    pack.write_all(p.as_bytes().as_slice()).unwrap()
                }
                pack
            }
        };
    }

    /// Makes a structured HoliumPack from its binarized version.
    ///
    /// # Panics
    ///
    /// TODO
    pub fn from_bytes(_buf: Vec<u8>) -> HoliumPack {
        todo!()
    }
}