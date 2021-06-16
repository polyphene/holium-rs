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
fn int_0_is_valid_holium_pack() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_sint(&mut pack, 0).unwrap();
    assert!(pack.validate());
}

#[test]
fn int_0_should_take_1_byte() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_sint(&mut pack, 0).unwrap();
    assert_eq!(1, pack.len());
}

#[test]
fn int_127_is_valid_holium_pack() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_sint(&mut pack, 127).unwrap();
    assert!(pack.validate());
}

#[test]
fn int_127_should_take_1_byte() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_sint(&mut pack, 127).unwrap();
    assert_eq!(1, pack.len());
}

#[test]
fn int_minus_1_is_valid_holium_pack() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_sint(&mut pack, -1).unwrap();
    assert!(pack.validate());
}

#[test]
fn int_minus_1_should_take_1_byte() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_sint(&mut pack, -1).unwrap();
    assert_eq!(1, pack.len());
}

#[test]
fn int_minus_32_is_valid_holium_pack() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_sint(&mut pack, -32).unwrap();
    assert!(pack.validate());
}

#[test]
fn int_minus_32_should_take_1_byte() {
    let mut pack: HoliumPack = Vec::new();
    rmp::encode::write_sint(&mut pack, -32).unwrap();
    assert_eq!(1, pack.len());
}