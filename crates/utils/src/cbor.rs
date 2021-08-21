//! This module features some CBOR helper methods, often missing from popular crates and mostly
//! related to the creation or parsing of major type and count prefixes.

/// Return a valid byte string header given a string length
pub fn create_cbor_byte_string_header(payload_len: u64) -> Vec<u8> {
    create_cbor_string_header(true, payload_len)
}

/// Return a valid text string header given a string length
pub fn create_cbor_text_string_header(payload_len: u64) -> Vec<u8> {
    create_cbor_string_header(false, payload_len)
}

/// Return a valid string header given a string length
fn create_cbor_string_header(is_byte_string: bool, payload_len: u64) -> Vec<u8> {
    // Match payload length to CBOR string short and eventual extended counts
    let (short_count, extended_count): (u8, Vec<u8>) = match payload_len {
        0..=23 => (payload_len as u8, vec![]),
        24..=0xff => (24, (payload_len as u8).to_be_bytes().to_vec()),
        0x100..=0xffff => (25, (payload_len as u16).to_be_bytes().to_vec()),
        0x10000..=0xffffffff => (26, (payload_len as u32).to_be_bytes().to_vec()),
        0x100000000..=0xffffffffffffffff => (27, (payload_len as u64).to_be_bytes().to_vec()),
    };
    // Build the header
    let first_byte: u8 = (32 * if is_byte_string { 2 } else { 3 }) + short_count;
    let mut header = vec![first_byte];
    header.extend_from_slice(extended_count.as_slice());
    // return
    header
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cbor_header_on_both_string_major_types() {
        assert_eq!(
            create_cbor_string_header(true, 3),
            vec![0b010_00011]
        );
        assert_eq!(
            create_cbor_string_header(false, 3),
            vec![0b011_00011]
        );
    }

    #[test]
    fn test_create_cbor_string_header_in_all_extended_count_cases() {
        assert_eq!(
            create_cbor_string_header(true, 50),
            vec![0b010_11000, 50]
        );
        assert_eq!(
            create_cbor_string_header(true, 500),
            vec![0b010_11001, 1, 244]
        );
        assert_eq!(
            create_cbor_string_header(true, 500000),
            vec![0b010_11010, 0, 7, 161, 32]
        );
        assert_eq!(
            create_cbor_string_header(true, 5000000000),
            vec![0b010_11011, 0, 0 , 0, 1, 42, 5 , 242, 0]
        );
    }

    #[test]
    fn test_create_cbor_string_header_on_edge_cases() {
        assert_eq!(
            create_cbor_string_header(true, 0),
            vec![0b010_00000]
        );
        assert_eq!(
            create_cbor_string_header(true, 23),
            vec![0b010_10111]
        );
        assert_eq!(
            create_cbor_string_header(true, 24),
            vec![0b010_11000, 24]
        );
        assert_eq!(
            create_cbor_string_header(true, 255),
            vec![0b010_11000, 255]
        );
        assert_eq!(
            create_cbor_string_header(true, 256),
            vec![0b010_11001, 1,0]
        );
        assert_eq!(
            create_cbor_string_header(true, 65535),
            vec![0b010_11001, 255,255]
        );
        assert_eq!(
            create_cbor_string_header(true, 65536),
            vec![0b010_11010, 0,1,0,0]
        );
        assert_eq!(
            create_cbor_string_header(true, 4294967295),
            vec![0b010_11010, 255,255,255,255]
        );
        assert_eq!(
            create_cbor_string_header(true, 4294967296),
            vec![0b010_11011, 0,0,0,1,0,0,0,0]
        );
    }
}
