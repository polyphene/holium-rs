use anyhow::{Context, Result};
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_cbor::to_vec;

use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("non existing major type")]
    NonExistingMajorType,
    #[error("holium cbor does not handle maps")]
    NotHandlingMap,
    #[error("holium cbor should have root of array type")]
    RootNotArray,
    #[error("could not read byte at offset: {0}")]
    NonReadableByte(u64),
    #[error("could not seek to offset")]
    FailToSeekToOffset,
    #[error("could not retrieve cursor position")]
    FailToGetCursorPosition,
    #[error("unhandled data details, currently handling data up to 64 bits of length")]
    UnhandledDataDetails,
    #[error("data details in cbor header wrongly encoded")]
    BadCborHeader,
}

/// [`MajorType`] represents cbor major types that can be found in the HoliumCbor format
#[derive(Clone, Copy, Debug)]
pub enum MajorType {
    Unsigned,
    Negative,
    Bytes,
    String,
    Array,
    Map,
    SimpleValues,
}

impl MajorType {
    fn is_array(&self) -> bool {
        match self {
            MajorType::Array => true,
            _ => false,
        }
    }

    fn is_map(&self) -> bool {
        match self {
            MajorType::Map => true,
            _ => false
        }
    }
}

trait ParseHoliumCbor {
    // To implement to define cursor on reader
    fn as_cursor(&self) -> Cursor<&[u8]>;

    fn read_complete_cbor(&self) -> Result<Vec<((MajorType, u64, u64, u64))>> {
        let mut buff = self.as_cursor();

        let mut elements: Vec<((MajorType, u64, u64, u64))> = vec![];

        let (major_type, header_offset, data_offset, data_size) = read_header(&mut buff)?;

        elements.push((major_type, header_offset, data_offset, data_size));

        if !major_type.is_array() {
            return Err(Error::RootNotArray.into());
        }

        let mut elements_details =
            read_recursive_elements_detail(&mut buff, major_type, data_offset, data_size).unwrap();

        elements.append(&mut elements_details);

        Ok(elements)
    }
}

fn get_cursor_position<R: Read + Seek>(reader: &mut R) -> Result<u64> {
    reader
        .stream_position()
        .context(Error::FailToGetCursorPosition)
}

/// Read returning its major type, its data byte offset and size
fn read_header<R: Read + Seek>(reader: &mut R) -> Result<(MajorType, u64, u64, u64)> {
    // Save he&der offset for later use
    let header_offset = get_cursor_position(reader)?;

    // read first byte
    let mut first_byte_buffer = [0];
    reader
        .read(&mut first_byte_buffer[..])
        .context(Error::NonReadableByte(get_cursor_position(reader)?))?;

    // Retrieving major type from the first 3 bits
    let major_type = match first_byte_buffer[0] >> 5 {
        0 => MajorType::Unsigned,
        1 => MajorType::Negative,
        2 => MajorType::Bytes,
        3 => MajorType::String,
        4 => MajorType::Array,
        5 => MajorType::Map,
        7 => MajorType::SimpleValues,
        _ => return Err(Error::NonExistingMajorType.into()),
    };

    // Retrieving value for 5 next bits, acceding to data details
    let data_details = first_byte_buffer[0] & 0x1F;

    let (data_offset, data_size) =
        read_data_size(reader, &major_type, header_offset, data_details)?;

    Ok((major_type, header_offset, data_offset, data_size))
}

/// Read data size from data details in Cbor header. Returns a tuple with data offset and data length
/// in bytes.
fn read_data_size<R: Read + Seek>(
    reader: &mut R,
    major_type: &MajorType,
    header_offset: u64,
    data_details: u8,
) -> Result<(u64, u64)> {
    match major_type {
        MajorType::Bytes | MajorType::String | MajorType::Array | MajorType::Map => {
            // As specified in CBOR, if sum of leftover bits >= 24 then information can be found on other bytes
            let additional_bytes_to_read = match data_details {
                0..=23 => return Ok((header_offset + 1, data_details as u64)),
                24 => 1,
                25 => 2,
                26 => 4,
                27 => 8,
                _ => return Err(Error::UnhandledDataDetails.into()),
            };

            // Get current offset
            let complementary_bytes_offset = header_offset + 1;

            // read desired bytes
            let mut bytes_buffer = vec![0; additional_bytes_to_read];
            reader
                .read(&mut bytes_buffer[..])
                .context(Error::NonReadableByte(get_cursor_position(reader)?))?;

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
                Err(Error::BadCborHeader.into())
            } else {
                // Compute data offset
                let data_offset = complementary_bytes_offset + additional_bytes_to_read as u64;

                Ok((data_offset, data_size))
            }
        }
        MajorType::Unsigned | MajorType::Negative => match data_details {
            0..=23 => Ok((header_offset, 1)),
            24 => Ok((header_offset + 1, 1)),
            25 => Ok((header_offset + 1, 2)),
            26 => Ok((header_offset + 1, 4)),
            27 => Ok((header_offset + 1, 8)),
            _ => return Err(Error::UnhandledDataDetails.into()),
        },
        MajorType::SimpleValues => Ok((header_offset, 1)),
    }
}

fn read_recursive_elements_detail<R: Read + Seek>(
    reader: &mut R,
    major_type: MajorType,
    data_offset: u64,
    data_size: u64,
) -> Result<Vec<((MajorType, u64, u64, u64))>> {
    // Initialize returned vec
    let mut elements_details: Vec<((MajorType, u64, u64, u64))> = vec![];
    // Set current position at data offset
    reader
        .seek(SeekFrom::Start(data_offset))
        .context(Error::FailToSeekToOffset)?;

    let mut elements_to_parse = data_size as u128;

    // For maps we have key/value to find so *2
    if major_type.is_map() {
        elements_to_parse *= 2;
    }

    for _ in 0..elements_to_parse {
        // Get element details
        let (element_major_type, header_offset, element_data_offset, element_data_size) =
            read_header(reader)?;

        // Add element to details
        elements_details.push((
            element_major_type,
            header_offset,
            element_data_offset,
            element_data_size,
        ));

        // If element is array, retrieve his elements size
        if element_major_type.is_array() || element_major_type.is_map() {
            let mut recursive_elements_details = read_recursive_elements_detail(
                reader,
                MajorType::Array,
                element_data_offset,
                element_data_size,
            )?;

            // Add element to details list
            elements_details.append(&mut recursive_elements_details);
        }

        // Set current position at next element offset
        let (_, _, element_data_offset, element_data_size) =
            elements_details[elements_details.len() - 1];
        reader
            .seek(SeekFrom::Start(element_data_offset + element_data_size))
            .context(Error::FailToSeekToOffset)?;
    }
    Ok(elements_details)
}