extern crate rmp;

pub type HoliumPack = Vec<u8>;

pub trait Validatable {
    fn validate(&self) -> bool;
}

impl Validatable for HoliumPack {
    fn validate(&self) -> bool {
        false
    }
}