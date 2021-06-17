extern crate rmp;

use rmp::decode::read_marker;
use rmp::Marker;

pub type HoliumPack = Vec<u8>;

pub trait Validatable {
    fn validate(&self) -> bool;
}

impl Validatable for HoliumPack {
    fn validate(&self) -> bool {
        match read_marker(&mut &self[..]).unwrap() {
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
            | Marker::Bin32
            | Marker::Array16
            | Marker::Array32 => true,
            Marker::FixArray(u8) => u8 < 15,
            Marker::FixStr(u8) => u8 < 32,
            Marker::FixPos(u8) => u8 < 128,
            Marker::FixNeg(i8) => -32 <= i8 && i8 < 0,
            _ => false
        }
    }
}