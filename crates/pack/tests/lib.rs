mod store;

extern crate holium_pack;
extern crate hex;

use holium_pack::HoliumPack;

#[test]
fn nil_msg_pack_is_valid_holium_pack() {
    let mut msg_pack: Vec<u8> = Vec::new();
    rmp::encode::write_nil(&mut msg_pack).unwrap();
    let holium_pack = HoliumPack::new_primitive(msg_pack);
    // also test the shape of the message itself
    match holium_pack {
        HoliumPack::Primitive(buf) => assert_eq!(vec![0xc0], buf),
        _ => panic!()
    }
}

#[test]
fn false_bool_msg_pack_is_valid_holium_pack() {
    let mut msg_pack: Vec<u8> = Vec::new();
    rmp::encode::write_bool(&mut msg_pack, false).unwrap();
    let holium_pack = HoliumPack::new_primitive(msg_pack);
    // also test the shape of the message itself
    match holium_pack {
        HoliumPack::Primitive(buf) => assert_eq!(vec![0xc2], buf),
        _ => panic!()
    }
}

#[test]
fn true_bool_msg_pack_is_valid_holium_pack() {
    let mut msg_pack: Vec<u8> = Vec::new();
    rmp::encode::write_bool(&mut msg_pack, true).unwrap();
    let holium_pack = HoliumPack::new_primitive(msg_pack);
    // also test the shape of the message itself
    match holium_pack {
        HoliumPack::Primitive(buf) => assert_eq!(vec![0xc3], buf),
        _ => panic!()
    }
}

#[test]
fn int_2_pow_64_minus_1_should_take_9_bytes() {
    // TODO
}

#[test]
fn int_2_pow_32_should_take_9_bytes() {
    // TODO
}

#[test]
fn int_2_pow_32_minus_1_should_take_5_bytes() {
    // TODO
}

#[test]
fn int_2_pow_16_should_take_5_bytes() {
    // TODO
}

#[test]
fn int_2_pow_16_minus_1_should_take_3_bytes() {
    // TODO
}

#[test]
fn int_2_pow_8_should_take_3_bytes() {
    // TODO
}

#[test]
fn int_2_pow_8_minus_1_should_take_2_bytes() {
    // TODO
}

#[test]
fn int_2_pow_7_should_take_2_bytes() {
    // TODO
}

#[test]
fn int_2_pow_7_minus_1_should_take_1_byte() {
    let mut msg_pack: Vec<u8> = Vec::new();
    rmp::encode::write_sint(&mut msg_pack, 127).unwrap();
    let holium_pack = HoliumPack::new_primitive(msg_pack);
    match holium_pack {
        HoliumPack::Primitive(buf) => assert_eq!(1, buf.len()),
        _ => panic!()
    }
    // TODO fix
    // try with larger sizes
    // pack.clear();
    // rmp::encode::write_u8(&mut pack, 127).unwrap();
    // assert_eq!(2, pack.len());
    // assert!(!pack.validate());
}

#[test]
fn int_0_should_take_1_byte() {
    let mut msg_pack: Vec<u8> = Vec::new();
    rmp::encode::write_sint(&mut msg_pack, 0).unwrap();
    let holium_pack = HoliumPack::new_primitive(msg_pack);
    match holium_pack {
        HoliumPack::Primitive(buf) => assert_eq!(1, buf.len()),
        _ => panic!()
    }
    // TODO fix
    // try with larger sizes
    // pack.clear();
    // rmp::encode::write_u8(&mut pack, 0).unwrap();
    // assert_eq!(2, pack.len());
    // assert!(!pack.validate());
}

#[test]
fn int_minus_1_should_take_1_byte() {
    let mut msg_pack: Vec<u8> = Vec::new();
    rmp::encode::write_sint(&mut msg_pack, -1).unwrap();
    let holium_pack = HoliumPack::new_primitive(msg_pack);
    match holium_pack {
        HoliumPack::Primitive(buf) => assert_eq!(1, buf.len()),
        _ => panic!()
    }
    // TODO fix
    // try with larger sizes
    // pack.clear();
    // rmp::encode::write_i8(&mut pack, -1).unwrap();
    // assert_eq!(2, pack.len());
    // assert!(!pack.validate());
}

#[test]
fn int_minus_2_pow_5_should_take_1_byte() {
    let mut msg_pack: Vec<u8> = Vec::new();
    rmp::encode::write_sint(&mut msg_pack, -32).unwrap();
    let holium_pack = HoliumPack::new_primitive(msg_pack);
    match holium_pack {
        HoliumPack::Primitive(buf) => assert_eq!(1, buf.len()),
        _ => panic!()
    }
    // TODO fix
    // try with larger sizes
    // pack.clear();
    // rmp::encode::write_i8(&mut pack, -32).unwrap();
    // assert_eq!(2, pack.len());
    // assert!(!pack.validate());
}

#[test]
fn int_minus_2_pow_5_minus_1_should_take_2_bytes() {
    // TODO
}

#[test]
fn int_minus_2_pow_7_should_take_2_bytes() {
    // TODO
}

#[test]
fn int_minus_2_pow_7_minus_1_should_take_3_bytes() {
    // TODO
}

#[test]
fn int_minus_2_pow_15_should_take_3_bytes() {
    // TODO
}

#[test]
fn int_minus_2_pow_15_minus_1_should_take_5_bytes() {
    // TODO
}

#[test]
fn int_minus_2_pow_31_should_take_5_bytes() {
    // TODO
}

#[test]
fn int_minus_2_pow_31_minus_1_should_take_9_bytes() {
    // TODO
}

#[test]
fn int_minus_2_pow_63_should_take_9_bytes() {
    // TODO
}

// TODO add tests on the representation of floats in Holium packs

// TODO add tests on the representation of strings (for each of the 4 possible size methods) in Holium packs

// TODO add tests on the representation of binaries (for each of the 3 possible size methods) in Holium packs

// TODO add tests on the representation of arrays (for each of the 3 possible size methods) in Holium packs
// verifying that all elements of any array should be other packs (not CIDs)

// TODO add tests on the non-representation of maps (for each of the 3 possible size methods) in Holium packs


// TODO add tests on the representation of primitive type values in Holium packs as bytes

// TODO add tests on the representation of arrays (for each of the 3 possible size methods) in Holium packs as bytes
// verifying that **all elements of any array should only be CIDs**