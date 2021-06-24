use holium_pack::HoliumPack;
use holium_pack::store::FragmentDecomposition;

#[test]
fn nil_holium_pack_cid_is_deterministic() {
    let mut msg_pack: Vec<u8> = Vec::new();
    rmp::encode::write_nil(&mut msg_pack).unwrap();
    let holium_pack = HoliumPack::new_primitive(msg_pack);
    let fd = FragmentDecomposition::from_holium_pack(holium_pack);
    assert_eq!(
        fd.cid().to_vec(),
        hex::decode("4e00a27d72b9cafbd6b13c870f1c25bf6f819f23e16a2a0b9f6aaffb06c45843").unwrap()
    );
}

#[test]
fn array_based_holium_packs_cids_are_deterministic() {
    // TODO
}