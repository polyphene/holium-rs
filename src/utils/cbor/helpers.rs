use crate::utils::cbor::as_holium_cbor::{MajorType, RecursiveType, ScalarType};
use anyhow::{Context, Result};
use std::io::{Read, Seek, SeekFrom};

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("non existing major type")]
    NonExistingMajorType,
    #[error("holium cbor should have root of array type")]
    RootNotArray,
    #[error("could not read byte at offset: {0}")]
    NonReadableByte(u64),
    #[error("could not seek to offset")]
    FailToSeekToOffset,
    #[error("could not retrieve cursor position")]
    FailToGetCursorPosition,
    #[error("unhandled data details, currently handling counts up to 64 bits of length")]
    UnhandledDataDetails,
    #[error("data details in cbor header wrongly encoded")]
    BadCborHeader,
    #[error("major type is non recursive")]
    MajorTypeNonRecursive,
}

#[derive(thiserror::Error, Debug)]
pub enum SelectorError {
    #[error("non valid selector structure")]
    NonValidSelectorStructure,
    #[error("no node for given selector")]
    NoNodeFound,
    #[error("union can only be found at the root of a selector")]
    UnionOnlyAtRoot,
    #[error("failed to select data at tail for connection: {0}")]
    DataAtTailSelectionFailed(String),
    #[error("result data set empty after tail selector is applied for connection: {0}")]
    ResultDataSetEmptyAfterSelection(String),
}

#[derive(thiserror::Error, Debug)]
pub enum WriteError {
    #[error("non compatible selectors for connection: {0}")]
    NonCompatibleSelectors(String),
    #[error("tail and head selectors union does not have the same number of elements in connection: {0}")]
    DifferentUnionLength(String),
    #[error("union should only be applied at root level for a Holium selector")]
    UnionOnlyAtRootLevel,
    #[error("tried to apply an index selection on a declared leaf in the tree")]
    IndexSelectionOnLeaf,
    #[error("tried to apply a range selection on a declared leaf in the tree")]
    RangeSelectionOnLeaf,
    #[error("index already taken by another element")]
    IndexAlreadyTaken,
    #[error("data set length is not equal to range length")]
    DatasetLengthUnequalRangeLength,
    #[error("no data in dataset")]
    NoDataInDataSet,
    #[error("no node at given index")]
    NoNodeAtIndex,
    #[error("expected children for recursive type")]
    ExpectedChildrenForRecursiveType,
    #[error("expected cbor data for recursive type")]
    ExpectedCborDataForRecursiveType,
    #[error("all children of a recursive node should have an index")]
    NoIndexOnChild,
    #[error("failed to copy data for connection: {0}")]
    DataCopyFailed(String),
    #[error("failed to generate valid cbor object")]
    CborGenerationFailed,
}

fn get_cursor_position<R: Read + Seek>(reader: &mut R) -> Result<u64> {
    reader
        .stream_position()
        .context(ParseError::FailToGetCursorPosition)
}

/// Read returning its major type, its data byte offset and size
pub fn read_header<R: Read + Seek>(reader: &mut R, header_offset: u64) -> Result<MajorType> {
    // Seek header offset
    reader
        .seek(SeekFrom::Start(header_offset))
        .context(ParseError::FailToSeekToOffset)?;

    // read first byte
    let mut first_byte_buffer = [0];
    reader
        .read(&mut first_byte_buffer[..])
        .context(ParseError::NonReadableByte(get_cursor_position(reader)?))?;

    // Major type information on first 3 bits
    let major_type_number = first_byte_buffer[0] >> 5;

    // Retrieving value for 5 last bits, acceding to data details
    let data_details = first_byte_buffer[0] & 0x1F;

    let (data_offset, data_size) =
        read_data_size(reader, major_type_number, header_offset, data_details)?;

    // Retrieving major type from the first 3 bits
    Ok(match major_type_number {
        0 => MajorType::Unsigned(ScalarType {
            header_offset,
            data_offset,
            data_size,
        }),
        1 => MajorType::Negative(ScalarType {
            header_offset,
            data_offset,
            data_size,
        }),
        2 => MajorType::Bytes(ScalarType {
            header_offset,
            data_offset,
            data_size,
        }),
        3 => MajorType::String(ScalarType {
            header_offset,
            data_offset,
            data_size,
        }),
        4 => MajorType::Array(RecursiveType {
            header_offset,
            data_offset,
            nbr_elements: data_size as usize,
            elements: vec![],
        }),
        5 => MajorType::Map(RecursiveType {
            header_offset,
            data_offset,
            nbr_elements: data_size as usize * 2,
            elements: vec![],
        }),
        7 => MajorType::SimpleValues(ScalarType {
            header_offset,
            data_offset,
            data_size,
        }),
        _ => return Err(ParseError::NonExistingMajorType.into()),
    })
}

/// Read data size from data details in Cbor header. Returns a tuple with data offset and data length
/// in bytes.
fn read_data_size<R: Read + Seek>(
    reader: &mut R,
    major_type: u8,
    header_offset: u64,
    data_details: u8,
) -> Result<(Option<u64>, u64)> {
    match major_type {
        // Major type is either unsigned or negative
        0 | 1 => match data_details {
            // In that case the data is directly contained in the header byte
            0..=23 => Ok((Some(header_offset), 1)),
            // Number data starts next offset and is 1 byte long
            24 => Ok((Some(header_offset + 1), 1)),
            // Number data starts next offset and is 2 byte long
            25 => Ok((Some(header_offset + 1), 2)),
            // Number data starts next offset and is 4 byte long
            26 => Ok((Some(header_offset + 1), 4)),
            // Number data starts next offset and is 8 byte long
            27 => Ok((Some(header_offset + 1), 8)),
            _ => return Err(ParseError::UnhandledDataDetails.into()),
        },
        // Major type is either bytes, string, array or map
        2 | 3 | 4 | 5 => {
            // As specified in CBOR, if sum of leftover bits >= 24 then information can be found on other bytes
            let additional_bytes_to_read = match data_details {
                // If data_details is 0 then there is no data to be found in our element. There is no
                // data offset and data size is 0
                0 => return Ok((None, data_details as u64)),
                // Case where data length can be directly found on header byte, returning data offset
                // and data details which is also data length
                1..=23 => return Ok((Some(header_offset + 1), data_details as u64)),
                // Length can be found on 1 byte after the header offset
                24 => 1,
                // Length can be found on 2 byte after the header offset
                25 => 2,
                // Length can be found on 4 byte after the header offset
                26 => 4,
                // Length can be found on 8 byte after the header offset
                27 => 8,
                _ => return Err(ParseError::UnhandledDataDetails.into()),
            };

            // Get current offset
            let complementary_bytes_offset = header_offset + 1;

            // Set current position at header offset + 1
            reader
                .seek(SeekFrom::Start(complementary_bytes_offset))
                .context(ParseError::FailToSeekToOffset)?;

            // read desired bytes
            let mut bytes_buffer = vec![0; additional_bytes_to_read];
            reader
                .read(&mut bytes_buffer[..])
                .context(ParseError::NonReadableByte(get_cursor_position(reader)?))?;

            // Currently handling up to 64 bits for length
            let mut data_size = 0 as u64;

            // Calculate data size based on bytes
            for byte in &bytes_buffer {
                data_size <<= 8;
                data_size += *byte as u64;
            }

            // If cbor header badly encoded return error
            if (additional_bytes_to_read == 1 && data_size < 24)
                || data_size < (1u64 << (8 * (additional_bytes_to_read >> 1)))
            {
                Err(ParseError::BadCborHeader.into())
            } else {
                // Compute data offset
                let data_offset = complementary_bytes_offset + additional_bytes_to_read as u64;

                Ok((Some(data_offset), data_size))
            }
        }
        7 => Ok((Some(header_offset), 1)),
        _ => return Err(ParseError::NonExistingMajorType.into()),
    }
}

pub fn fetch_recursive_elements_detail<R: Read + Seek>(
    reader: &mut R,
    recursive_type: &mut RecursiveType,
) -> Result<()> {
    // if no elements are expected then return
    if recursive_type.nbr_elements == 0usize {
        return Ok(());
    }

    // Define data offset
    let mut data_offset = recursive_type.data_offset.unwrap();

    for _ in 0..recursive_type.nbr_elements {
        // Get element details
        let mut element_major_type = read_header(reader, data_offset)?;

        // If element is array or map, retrieve his elements size
        match &mut element_major_type {
            MajorType::Array(recursive_element) | MajorType::Map(recursive_element) => {
                fetch_recursive_elements_detail(reader, recursive_element)?
            }
            _ => {}
        }

        recursive_type.elements.push(element_major_type);

        // Set current position at next element offset
        data_offset = recursive_type.elements[recursive_type.elements.len() - 1].next_offset();
    }

    Ok(())
}

/// [retrieve_cbor_in_reader] will fetch cbor data in reader based on a [MajorType] structure containing
/// structural information of where the data is in the reader
pub fn retrieve_cbor_in_reader<R: Read + Seek>(
    reader: &mut R,
    to_retrieve: &[Vec<MajorType>],
) -> Result<Vec<Vec<Vec<u8>>>> {
    let mut retrieved_data: Vec<Vec<Vec<u8>>> = vec![];

    // Iterate o multiple set of data to retrieve
    for major_types_set in to_retrieve.iter() {
        let mut data_set: Vec<Vec<u8>> = vec![];
        // Iterate on data in each set
        for major_type in major_types_set.iter() {
            // Get major type details
            let (header_offset, size) = major_type.details();

            reader
                .seek(SeekFrom::Start(header_offset))
                .context(ParseError::FailToSeekToOffset)?;

            // read bytes
            let mut byte_buffer = vec![0; size];
            reader
                .read(&mut byte_buffer[..])
                .context(ParseError::NonReadableByte(get_cursor_position(reader)?))?;

            data_set.push(byte_buffer);
        }
        retrieved_data.push(data_set);
    }

    Ok(retrieved_data)
}

/// Generates a cbor header for an array major type based on its size
pub fn generate_array_cbor_header(size: u64) -> Vec<u8> {
    let (mut first_byte, shift) = match size {
        0..=23 => (size as u8, 0),
        24..=0xFF => (24, 1),
        0x100..=0xFFFF => (25, 2),
        0x10000..=0xFFFF_FFFF => (26, 4),
        _ => (27, 8),
    };
    first_byte |= 128;

    let mut buff: Vec<u8> = vec![first_byte];

    for i in (0..shift).rev() {
        buff.push((size >> (i * 8)) as u8);
    }

    buff
}
