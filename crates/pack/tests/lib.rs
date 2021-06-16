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

#[test]
fn false_bool_msg_pack_is_valid_holium_pack() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_bool(&mut pack, false).unwrap();
    assert!(pack.validate());
    // also test the shape of the message itself
    assert_eq!(vec![0xc2], pack);
}

#[test]
fn true_bool_msg_pack_is_valid_holium_pack() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_bool(&mut pack, true).unwrap();
    assert!(pack.validate());
    // also test the shape of the message itself
    assert_eq!(vec![0xc3], pack);
}

#[test]
fn positive_fixint_msg_pack_is_valid_holium_pack() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_pfix(&mut pack, 0).unwrap();
    assert!(pack.validate());
    assert_eq!(vec![0x00], pack);
    pack.clear();
    rmp::encode::write_pfix(&mut pack, 0x7f).unwrap();
    assert!(pack.validate());
    assert_eq!(vec![0x7f], pack);
}