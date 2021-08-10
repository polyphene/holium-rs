use serde_cbor::Value as CborValue;

use crate::data::importer::Importable;

impl Importable for CborValue {
    fn to_cbor(&self) -> CborValue {
        self.clone()
    }
}
