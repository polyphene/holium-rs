extern crate rmp;

use rmp::decode::read_marker;
use rmp::Marker;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;

static ZLIB_COMPRESSION: Compression = Compression::new(6);

type HoliumFragmentCID = [u8; 32];

pub trait HoliumTypes {
    fn is_holium_primitive_msg(&self) -> bool;

    fn is_holium_array_pack(&self) -> bool;
    fn is_holium_pack(&self) -> bool;

    fn is_holium_array_fragment(&self) -> bool;
    fn is_holium_fragment(&self) -> bool;
}

pub trait HoliumFragments {
    fn compute_cid(&self) -> HoliumFragmentCID;
}

impl HoliumTypes for Vec<u8> {
    fn is_holium_primitive_msg(&self) -> bool {
        match read_marker(&mut &self[..]).unwrap() {
            Marker::Null
            | Marker::False
            | Marker::True
            | Marker::U8
            | Marker::U16
            | Marker::U32
            | Marker::U64
            | Marker::I8
            | Marker::I16
            | Marker::I32
            | Marker::I64
            | Marker::F32
            | Marker::F64
            | Marker::Str8
            | Marker::Str16
            | Marker::Str32
            | Marker::Bin8
            | Marker::Bin16
            | Marker::Bin32 => true,
            Marker::FixStr(u8) => u8 < 32,
            Marker::FixPos(u8) => u8 < 128,
            Marker::FixNeg(i8) => -32 <= i8 && i8 < 0,
            _ => false
        }
    }

    fn is_holium_array_pack(&self) -> bool {
        match read_marker(&mut &self[..]).unwrap() {
            Marker::Array16 => true,
            Marker::Array32 => true,
            Marker::FixArray(u8) => u8 < 15,
            _ => false
        }
    }

    fn is_holium_pack(&self) -> bool {
        self.is_holium_primitive_msg() || self.is_holium_array_pack()
    }

    fn is_holium_array_fragment(&self) -> bool {
        match read_marker(&mut &self[..]).unwrap() {
            Marker::Array16 => true,
            Marker::Array32 => true,
            Marker::FixArray(u8) => u8 < 15,
            _ => false
        }
    }

    fn is_holium_fragment(&self) -> bool {
        self.is_holium_primitive_msg() || self.is_holium_array_fragment()
    }
}


impl HoliumFragments for Vec<u8> {
    fn compute_cid(&self) -> HoliumFragmentCID {
        if self.is_holium_primitive_msg() {
            let mut zlib_encoder = ZlibEncoder::new(Vec::new(), ZLIB_COMPRESSION);
            zlib_encoder.write_all(self).unwrap();
            let compressed_bytes = zlib_encoder.finish().unwrap();
            let cid = blake3::hash(&*compressed_bytes);
            return *cid.as_bytes();
        }
        panic!()
    }
}