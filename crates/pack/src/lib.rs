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
            Marker::Null => true,
            _ => false
        }
    }
}