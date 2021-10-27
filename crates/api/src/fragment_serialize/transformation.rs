use std::io::Read;

use anyhow::{Context, Result};

use crate::fragment_serialize::{FragmentedDataDeserResult, HoliumDeserializable};
use crate::transformation::Transformation;
use holium_utils::cbor::WASM_MAGIC_NUMBER;

impl HoliumDeserializable for Transformation {
    // fn is_of_type<R: Read>(mut data_reader: R) -> Result<bool> {
    fn is_of_type<R: Read>(data_reader: &mut R) -> Result<bool> {
        // Read first few bytes
        // CBOR string header may take up to 9 bytes
        // WebAssembly magic code should then take for 4 bytes
        let mut first_bytes = [0u8; 13];
        data_reader.read(&mut first_bytes)
            .context("failed to read transformation file")?;
        // Check for the presence of a valid CBOR header
        let expected_magic_number: &[u8] = match first_bytes[0] {
            0b010_00000..=0b010_10111 => &first_bytes[1..5],
            0b010_11000 => &first_bytes[2..6],
            0b010_11001 => &first_bytes[3..7],
            0b010_11010 => &first_bytes[5..9],
            0b010_11011 => &first_bytes[9..13],
            _ => {return Ok(false)},
        };
        Ok(expected_magic_number == WASM_MAGIC_NUMBER)
    }

    fn value_from_bytes(data: &[u8]) -> FragmentedDataDeserResult<Box<Self>> {
        FragmentedDataDeserResult { value: Box::new(Transformation {}), links: vec![] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod test_is_of_type {
        use super::*;

        #[test]
        fn can_recognize_wrong_major_type() {
            let data = hex::decode("f6").unwrap();
            assert!(!Transformation::is_of_type(&mut data.as_slice()).unwrap())
        }

        #[test]
        fn can_recognize_missing_wasm_magic_number() {
            let data = hex::decode("480001020304050607").unwrap();
            assert!(!Transformation::is_of_type(&mut data.as_slice()).unwrap())
        }

        #[test]
        fn can_recognize_valid_transformation() {
            let data = hex::decode("58320061736D010000000105016000017F03020100070801046D61696E00000A07010500412A0F0B000A046E616D650203010000").unwrap();
            assert!(Transformation::is_of_type(&mut data.as_slice()).unwrap())
        }
    }
}