use anyhow::{Context, Result};
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_cbor::to_vec;

use crate::utils::interplanetary::kinds::selector::{Selector, SelectorEnvelope};
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
    #[error("major type is non recursive")]
    MajorTypeNonRecursive,
}

#[derive(thiserror::Error, Debug)]
enum SelectorError {
    #[error("non valid selector structure")]
    NonValidSelectorStructure,
    #[error("no node for given selector")]
    NoNodeFound,
}

/// [`ScalarType`] contains all information relative to a scalar cbor major type in a cursor
#[derive(Clone, Copy, Debug)]
pub struct ScalarType {
    header_offset: u64,
    data_offset: Option<u64>,
    data_size: u64,
}

/// [`RecursiveType`] contains all information relative to a recursive cbor major type in a cursor
#[derive(Clone, Debug)]
pub struct RecursiveType {
    header_offset: u64,
    data_offset: Option<u64>,
    nbr_elements: usize,
    elements: Vec<MajorType>,
}

/// [`MajorType`] represents cbor major types that can be found in the HoliumCbor format.
#[derive(Clone, Debug)]
pub enum MajorType {
    Unsigned(ScalarType),
    Negative(ScalarType),
    Bytes(ScalarType),
    String(ScalarType),
    Array(RecursiveType),
    Map(RecursiveType),
    SimpleValues(ScalarType),
}

impl MajorType {
    fn is_array(&self) -> bool {
        match self {
            MajorType::Array(_) => true,
            _ => false,
        }
    }

    fn is_map(&self) -> bool {
        match self {
            MajorType::Map(_) => true,
            _ => false,
        }
    }

    /// Crawl through a major type and compute the next empty byte offset
    fn next_offset(&self) -> u64 {
        return match self {
            MajorType::Array(recursive) | MajorType::Map(recursive) => {
                // if no elements in recursive then return offset after header
                if recursive.nbr_elements == 0usize {
                    return recursive.header_offset + 1;
                }

                // if currently no elements in recursive but some element are expected then return
                // data offset. We can unwrap as we checked that some elements are expected.
                if recursive.elements.len() == 0usize {
                    return recursive.data_offset.unwrap();
                }

                // if some elements are already written then return offset after the last element
                recursive.elements[recursive.elements.len() - 1].next_offset()
            }
            MajorType::Unsigned(scalar)
            | MajorType::Negative(scalar)
            | MajorType::Bytes(scalar)
            | MajorType::String(scalar)
            | MajorType::SimpleValues(scalar) => {
                if let Some(data_offset) = scalar.data_offset {
                    data_offset + scalar.data_size;
                } else {
                    scalar.header_offset + 1;
                }
            }
        };
    }

    /// Return a reference to a child of a recursive major type
    fn child(&self, index: u64) -> Option<&MajorType> {
        return match self {
            MajorType::Array(recursive) | MajorType::Map(recursive) => {
                // if no elements in recursive or index is out of bounds return none
                if recursive.nbr_elements == 0usize || recursive.nbr_elements - 1 < index as usize {
                    return None;
                }

                // if some elements are already written then return offset after the last element
                Some(&recursive.elements[index as usize])
            }
            MajorType::Unsigned(scalar)
            | MajorType::Negative(scalar)
            | MajorType::Bytes(scalar)
            | MajorType::String(scalar)
            | MajorType::SimpleValues(scalar) => None,
        };
    }

    /// Find a major type by using a selector
    fn select(&self, selector: &Selector) -> Result<Vec<Vec<MajorType>>> {
        match selector {
            Selector::Matcher(_) => Ok(vec![vec![self.clone()]]),
            Selector::ExploreIndex(explore_index) => {
                let explored_major_type = self.child(explore_index.index);

                match explored_major_type {
                    Some(major_type) => Ok(major_type.select(&explore_index.next)?),
                    None => return Err(SelectorError::NoNodeFound.into()),
                }
            }
            Selector::ExploreRange(explore_range) => {
                let mut selected_major_types: Vec<MajorType> = vec![];

                for index in explore_range.start..explore_range.end {
                    // After a range we expect a matcher, otherwise error
                    if explore_range.next.is_matcher() {
                        return Err(SelectorError::NonValidSelectorStructure.into());
                    }

                    let explored_major_type = self.child(index);
                    match explored_major_type {
                        Some(major_type) => {
                            selected_major_types.push(major_type.clone());
                        }
                        None => return Err(SelectorError::NoNodeFound.into()),
                    }
                }

                Ok(vec![selected_major_types])
            }
            Selector::ExploreUnion(explore_union) => {
                let mut selectors_results: Vec<Vec<MajorType>> = vec![];
                let selectors = &explore_union.0;
                for selector in selectors.iter() {
                    selectors_results.append(&mut self.select(selector)?);
                }

                Ok(selectors_results)
            }
        }
    }
}

trait ParseHoliumCbor {
    // To implement to define cursor on reader
    fn as_cursor(&self) -> Cursor<&[u8]>;

    /// Give entire description of a holium cbor serialized object
    fn complete_cbor_structure(&self) -> Result<MajorType> {
        let mut buff = self.as_cursor();

        let mut major_type = read_header(&mut buff)?;

        if !major_type.is_array() {
            return Err(Error::RootNotArray.into());
        }
        read_recursive_elements_detail(&mut buff, &mut major_type).unwrap();

        Ok(major_type)
    }

    /// Select part of a holium cbor serialized object using a selector
    fn select_cbor_structure(
        &self,
        selector_envelope: &SelectorEnvelope,
    ) -> Result<Vec<Vec<MajorType>>> {
        let mut buff = self.as_cursor();

        let mut major_type = read_header(&mut buff)?;

        if !major_type.is_array() {
            return Err(Error::RootNotArray.into());
        }
        read_recursive_elements_detail(&mut buff, &mut major_type).unwrap();

        Ok(major_type.select(&selector_envelope.0)?)
    }
}

fn get_cursor_position<R: Read + Seek>(reader: &mut R) -> Result<u64> {
    reader
        .stream_position()
        .context(Error::FailToGetCursorPosition)
}
/// Read returning its major type, its data byte offset and size
fn read_header<R: Read + Seek>(reader: &mut R) -> Result<MajorType> {
    // Save he&der offset for later use
    let header_offset = get_cursor_position(reader)?;

    // read first byte
    let mut first_byte_buffer = [0];
    reader
        .read(&mut first_byte_buffer[..])
        .context(Error::NonReadableByte(get_cursor_position(reader)?))?;

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
        _ => return Err(Error::NonExistingMajorType.into()),
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
        0 | 1 => match data_details {
            0..=23 => Ok((Some(header_offset), 1)),
            24 => Ok((Some(header_offset + 1), 1)),
            25 => Ok((Some(header_offset + 1), 2)),
            26 => Ok((Some(header_offset + 1), 4)),
            27 => Ok((Some(header_offset + 1), 8)),
            _ => return Err(Error::UnhandledDataDetails.into()),
        },
        2 | 3 | 4 | 5 => {
            // As specified in CBOR, if sum of leftover bits >= 24 then information can be found on other bytes
            let additional_bytes_to_read = match data_details {
                0 => return Ok((None, data_details as u64)),
                1..=23 => return Ok((Some(header_offset + 1), data_details as u64)),
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

                Ok((Some(data_offset), data_size))
            }
        }
        7 => Ok((Some(header_offset), 1)),
        _ => return Err(Error::NonExistingMajorType.into()),
    }
}

fn read_recursive_elements_detail<R: Read + Seek>(
    reader: &mut R,
    major_type: &mut MajorType,
) -> Result<()> {
    // Check that major type is recursive
    match major_type {
        MajorType::Array(recursive) | MajorType::Map(recursive) => {
            // if no elements are expected then return
            if recursive.nbr_elements == 0usize {
                return Ok(());
            }

            // Set current position at data offset. We can unwrap as elements are expected.
            reader
                .seek(SeekFrom::Start(recursive.data_offset.unwrap()))
                .context(Error::FailToSeekToOffset)?;

            // Can unwrap we check previously that
            let mut elements_to_parse = recursive.nbr_elements;

            for _ in 0..elements_to_parse {
                // Get element details
                let mut element_major_type = read_header(reader)?;

                // If element is array, retrieve his elements size
                if element_major_type.is_array() || element_major_type.is_map() {
                    read_recursive_elements_detail(reader, &mut element_major_type)?;
                }

                recursive.elements.push(element_major_type);

                // Set current position at next element offset
                let next_offset = recursive.elements[recursive.elements.len() - 1].next_offset();

                reader
                    .seek(SeekFrom::Start(next_offset))
                    .context(Error::FailToSeekToOffset)?;
            }
        }
        _ => return Err(Error::MajorTypeNonRecursive.into()),
    }

    Ok(())
}
