use crate::utils::cbor::helpers::{
    fetch_recursive_elements_detail, read_header, retrieve_cbor_in_reader, ParseError,
    SelectorError,
};
use crate::utils::interplanetary::kinds::selector::{Selector, SelectorEnvelope};
use anyhow::Result;
use std::io::Cursor;

pub trait AsHoliumCbor {
    // To implement to define cursor on reader
    fn as_cursor(&self) -> Cursor<&[u8]>;

    /// Give entire description of a holium cbor serialized object
    fn complete_cbor_structure(&self) -> Result<MajorType> {
        let mut buff = self.as_cursor();

        let mut major_type = read_header(&mut buff, 0)?;

        match &mut major_type {
            MajorType::Array(recursive_type) => {
                fetch_recursive_elements_detail(&mut buff, recursive_type)?
            }
            _ => return Err(ParseError::RootNotArray.into()),
        }

        Ok(major_type)
    }

    /// Select part of a holium cbor serialized object using a selector
    fn select_cbor_structure(
        &self,
        selector_envelope: &SelectorEnvelope,
    ) -> Result<Vec<Vec<MajorType>>> {
        let mut buff = self.as_cursor();

        let mut major_type = read_header(&mut buff, 0)?;

        match &mut major_type {
            MajorType::Array(recursive_type) => {
                fetch_recursive_elements_detail(&mut buff, recursive_type)?
            }
            _ => return Err(ParseError::RootNotArray.into()),
        }

        Ok(major_type.select(&selector_envelope.0)?)
    }

    fn select_cbor(&self, selector_envelope: &SelectorEnvelope) -> Result<Vec<Vec<Vec<u8>>>> {
        let select_cbor_structure = self.select_cbor_structure(selector_envelope)?;

        let mut buff = self.as_cursor();

        retrieve_cbor_in_reader(&mut buff, &select_cbor_structure)
    }
}

/// [`ScalarType`] contains all information relative to a scalar cbor major type in a cursor
#[derive(Clone, Copy, Debug)]
pub struct ScalarType {
    pub header_offset: u64,
    pub data_offset: Option<u64>,
    pub data_size: u64,
}

/// [`RecursiveType`] contains all information relative to a recursive cbor major type in a cursor
#[derive(Clone, Debug)]
pub struct RecursiveType {
    pub header_offset: u64,
    pub data_offset: Option<u64>,
    // nbr_elements is a field that represents the number of awaited elements in our recursive major
    // type. It has to be based on the size given by a cbor header
    pub nbr_elements: usize,
    // elements contains all cbor details about children of the recursive major type. When reading
    // a major type from a cbor serialized value the elements detail will not be fetched. It has
    // to be done asynchronously.
    pub(crate) elements: Vec<MajorType>,
}

impl RecursiveType {
    /// Return a reference to a child of a recursive major type
    fn child(&self, index: usize) -> Result<&MajorType> {
        if self.nbr_elements == 0usize || self.nbr_elements - 1 < index {
            return Err(SelectorError::NoNodeFound.into());
        }

        // if some elements are already written then return offset after the last element
        Ok(&self.elements[index])
    }
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
    /// [details] returns the header offset for a given major type and its size in bytes. If no data then
    /// returns `(header_offset, 1)
    pub fn details(&self) -> (u64, usize) {
        match self {
            MajorType::Unsigned(scalar)
            | MajorType::Negative(scalar)
            | MajorType::Bytes(scalar)
            | MajorType::String(scalar)
            | MajorType::SimpleValues(scalar) => {
                return if scalar.data_offset.is_none() {
                    (scalar.header_offset, 1usize)
                } else {
                    (
                        scalar.header_offset,
                        (scalar.data_offset.unwrap() - scalar.header_offset + scalar.data_size)
                            as usize,
                    )
                }
            }
            MajorType::Array(recursive) | MajorType::Map(recursive) => {
                if recursive.nbr_elements == 0 {
                    return (recursive.header_offset, 1);
                }

                let mut recursive_size = 0usize;
                recursive_size +=
                    (recursive.data_offset.unwrap() - recursive.header_offset) as usize;

                for major_type in recursive.elements.iter() {
                    recursive_size += major_type.details().1;
                }

                (recursive.header_offset, recursive_size)
            }
        }
    }

    /// Crawl through a major type and compute the next empty byte offset
    pub fn next_offset(&self) -> u64 {
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
                return if let Some(data_offset) = scalar.data_offset {
                    data_offset + scalar.data_size
                } else {
                    scalar.header_offset + 1
                }
            }
        };
    }

    /// Find a major type by using a selector. Returned value is a list of data set description. If
    /// the selector contains a union (`|`) operator then we have multiple data sets. Then each
    /// data set contains one or multiple fetched [`MajorType`]
    pub fn select(&self, selector: &Selector) -> Result<Vec<Vec<MajorType>>> {
        match selector {
            Selector::Matcher(_) => Ok(vec![vec![self.clone()]]),
            Selector::ExploreIndex(explore_index) => match &self {
                MajorType::Array(recursive_type) | MajorType::Map(recursive_type) => {
                    let major_type = recursive_type.child(explore_index.index as usize)?;
                    Ok(major_type.select(&explore_index.next)?)
                }
                _ => Err(ParseError::MajorTypeNonRecursive.into()),
            },
            Selector::ExploreRange(explore_range) => {
                // After a range we expect a matcher, otherwise error
                if explore_range.next.is_matcher() {
                    return Err(SelectorError::NonValidSelectorStructure.into());
                }

                let mut selected_major_types: Vec<MajorType> =
                    Vec::with_capacity((explore_range.end - explore_range.start) as usize);

                match &self {
                    MajorType::Array(recursive_type) | MajorType::Map(recursive_type) => {
                        for index in explore_range.start..explore_range.end {
                            let major_type = recursive_type.child(index as usize)?;
                            selected_major_types.push(major_type.clone());
                        }
                    }
                    _ => return Err(ParseError::MajorTypeNonRecursive.into()),
                };

                Ok(vec![selected_major_types])
            }
            Selector::ExploreUnion(explore_union) => {
                let selectors = &explore_union.0;
                let mut selectors_results: Vec<Vec<MajorType>> =
                    Vec::with_capacity(selectors.len());

                for selector in selectors.iter() {
                    selectors_results.append(&mut self.select(selector)?);
                }

                Ok(selectors_results)
            }
        }
    }
}
