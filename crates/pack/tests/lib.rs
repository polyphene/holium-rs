extern crate holium_pack;
use holium_pack::{HoliumPack, Validatable};

#[test]
fn pack_is_bytes() {
    let _: HoliumPack = vec![0xc0, 0xc0];
}

#[test]
fn pack_can_be_validated() {
    let pack: HoliumPack = vec![0xc0, 0xc0];
    pack.validate();
}

#[test]
fn nil_msg_pack_is_valid_holium_pack() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_nil(&mut pack).unwrap();
    assert!(pack.validate());
    // also test the shape of the message itself
    assert_eq!(vec![0xc0], pack);
}