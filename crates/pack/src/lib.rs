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
            | Marker::True => true,
            Marker::FixPos(u8) => u8 < 128,
            Marker::FixNeg(i8) => -32 <= i8 && i8 < 0,
            _ => false
        }
    }
}