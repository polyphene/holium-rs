extern crate holium_pack as pack;
use pack::{HoliumPack};

#[test]
fn pack_is_bytes() {
    let _: HoliumPack = vec![0xc3, 0xc3];
}
