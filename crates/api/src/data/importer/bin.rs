use serde_cbor::Value as CborValue;

use crate::data::importer::Importable;

pub struct BinaryValue(Vec<u8>);

impl BinaryValue {
    pub fn new(buf: Vec<u8>) -> Self { BinaryValue(buf) }
}

impl Importable for BinaryValue {
    fn to_cbor(&self) -> CborValue {
        let content = self.0.clone();
        CborValue::Bytes(content)
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