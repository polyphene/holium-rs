use serde_cbor::Value as CborValue;

use crate::data::importer::Importable;

struct BinaryValue(Vec<u8>);

impl Importable for BinaryValue {
    fn to_cbor(self) -> CborValue {
        CborValue::Bytes(Vec::from(self.0))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_import_bin() {
        assert_eq!(
            BinaryValue(vec![0x00, 0xff]).to_cbor(),
            CborValue::Bytes(vec![0x00, 0xff])
        )
    }
}