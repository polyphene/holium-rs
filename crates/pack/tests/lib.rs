extern crate holium_pack;
use holium_pack::{HoliumPack, Validatable};

#[test]
fn pack_is_bytes() {
    let _: HoliumPack = vec![0xc3, 0xc3];
}

#[test]
fn pack_can_be_validated() {
    let pack: HoliumPack = vec![0xc3, 0xc3];
    assert!(pack.validate())
}
