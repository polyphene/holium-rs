use serde_cbor::Value as CborValue;

use crate::data::importer::Importable;

impl Importable for &[u8] {
    fn to_cbor(self) -> CborValue {
        CborValue::Bytes(Vec::from(self))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_import_bin() {
        assert_eq!(
            b"\x00\xff".to_cbor(),
            CborValue::Bytes(vec![0x00, 0xff])
        )
    }
}