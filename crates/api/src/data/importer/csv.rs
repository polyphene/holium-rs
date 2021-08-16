use csv::StringRecord;
use serde_cbor::Value as CborValue;

use crate::data::importer::Importable;

impl Importable for Vec<StringRecord> {
    fn to_cbor(&self) -> CborValue {
        let mut vec: Vec<CborValue> = Vec::new();
        for record in self {
            let cbor_value = CborValue::Array(
                record.iter().map(|v| CborValue::Text(String::from(v))).collect()
            );
            vec.push(cbor_value);
        }
        CborValue::Array(vec)
    }
}


#[cfg(test)]
mod tests {
    use csv::{ReaderBuilder, StringRecord};

    use super::*;

    #[test]
    fn can_import_csv() {
        // Initialize records
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader("a,b,c\nx,y,z".as_bytes());
        let records: Vec<StringRecord> = reader.records()
            .into_iter()
            .filter_map(|record| record.ok())
            .collect();
        // Test conversion
        assert_eq!(
            records.to_cbor(),
            CborValue::Array(vec![
                CborValue::Array(vec![
                    CborValue::Text(String::from("a")),
                    CborValue::Text(String::from("b")),
                    CborValue::Text(String::from("c")),
                ]),
                CborValue::Array(vec![
                    CborValue::Text(String::from("x")),
                    CborValue::Text(String::from("y")),
                    CborValue::Text(String::from("z")),
                ]),
            ])
        )
    }
}