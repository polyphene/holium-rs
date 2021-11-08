use anyhow::{Context, Result};
use std::borrow::Borrow;
use std::fmt::Debug;

use serde_cbor::to_vec;

use crate::utils::interplanetary::kinds::selector::{Selector, SelectorEnvelope};
use either::Either;
use either::Either::{Left, Right};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[derive(thiserror::Error, Debug)]
enum ParseError {
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
    #[error("union can only be found at the root of a selector")]
    UnionOnlyAtRoot,
}

#[derive(thiserror::Error, Debug)]
enum WriteError {
    #[error("non compatible selectors")]
    NonCompatibleSelectors,
    #[error("tail and head selectors union does not have the same number of elements")]
    DifferentUnionLength,
    #[error("union should only be applied at root level for a Holium selector")]
    UnionOnlyAtRootLevel,
    #[error("tried to apply an index selection on a declared leaf in the tree")]
    IndexSelectionOnLeaf,
    #[error("tried to apply a range selection on a declared leaf in the tree")]
    RangeSelectionOnLeaf,
    #[error("index already taken by another element")]
    IndexAlreadyTaken,
    #[error("data set length is not equal to range length")]
    DatasetLengthInequalRangeLength,
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
    #[error("index not allocated to one child")]
    IndexNotAllocatedToOneChild,
}

/**************************************************
 * MajorTypes
 **************************************************/

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

    /// [details] returns the header offset for a given major type and its size in bytes. If no data then
    /// returns `(header_offset, 1)
    fn details(&self) -> (u64, usize) {
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
                if scalar.data_offset.is_some() {
                    scalar.data_offset.unwrap() + scalar.data_size
                } else {
                    scalar.header_offset + 1
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
            MajorType::Unsigned(_)
            | MajorType::Negative(_)
            | MajorType::Bytes(_)
            | MajorType::String(_)
            | MajorType::SimpleValues(_) => None,
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
                    if !explore_range.next.is_matcher() {
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

// [Leaf] represent a data that is a leaf in a HoliumCbor data
#[derive(Clone, Debug)]
struct ScalarNode {
    index: u64,
    data: Vec<u8>,
}

/// [ScalarNode] represent a node in a HoliumCbor data
#[derive(Clone, Debug)]
struct RecursiveNode {
    index: Option<u64>,
    data: Either<Vec<u8>, Vec<HoliumCborNode>>,
}

impl RecursiveNode {
    fn has_child(&self, index: usize) -> Result<bool> {
        match &self.data {
            Right(children) => {
                let mut has_child = false;
                for c in children.iter() {
                    if c.get_index().ok_or(WriteError::NoIndexOnChild)? as usize == index {
                        has_child = true;
                    }
                }
                Ok(has_child)
            }
            _ => Err(WriteError::ExpectedChildrenForRecursiveType.into()),
        }
    }

    fn child_as_mut(&mut self, index: usize) -> Result<&mut HoliumCborNode> {
        match self.data.as_mut() {
            Right(children) => {
                // Not using find() as to propagate error on get_index()
                for c in children.iter_mut() {
                    if c.get_index().ok_or(WriteError::NoIndexOnChild)? as usize == index {
                        return Ok(c);
                    }
                }
                Err(WriteError::NoNodeAtIndex.into())
            }
            _ => Err(WriteError::ExpectedChildrenForRecursiveType.into()),
        }
    }

    fn push_child(&mut self, child: HoliumCborNode) -> Result<()> {
        // Making sure index is not already taken
        if self.has_child(child.get_index().ok_or(WriteError::NoIndexOnChild)? as usize)? {
            return Err(WriteError::IndexAlreadyTaken.into());
        }
        match self.data.as_mut() {
            Right(children) => Ok(children.push(child)),
            _ => Err(WriteError::ExpectedChildrenForRecursiveType.into()),
        }
    }
}

/// [HoliumCborConstructor] is a utility structure to create a HoliumCbor object at a later time
#[derive(Clone, Debug)]
enum HoliumCborNode {
    Leaf(ScalarNode),
    NonLeaf(RecursiveNode),
}

impl HoliumCborNode {
    fn new() -> Self {
        HoliumCborNode::NonLeaf(RecursiveNode {
            index: Default::default(),
            data: Right(vec![]),
        })
    }

    fn get_index(&self) -> Option<u64> {
        match self {
            HoliumCborNode::NonLeaf(node) => node.index,
            HoliumCborNode::Leaf(leaf) => Some(leaf.index),
        }
    }

    fn is_node(&self) -> bool {
        match self {
            HoliumCborNode::NonLeaf(_) => true,
            _ => false,
        }
    }

    /// Ingest data from a dataset based on a [Selector]
    fn ingest(&mut self, selector: &Selector, data_set: &mut [Vec<u8>]) -> Result<()> {
        match selector {
            Selector::ExploreIndex(explore_index) => {
                // Making sure that we are on a Node and not a leaf. This can be avoided if selector is
                // properly constructed
                match self {
                    HoliumCborNode::NonLeaf(node) => {
                        match explore_index.next.borrow() {
                            Selector::Matcher(_) => {
                                // Making sure index is not already taken
                                if node.has_child(explore_index.index as usize)? {
                                    return Err(WriteError::IndexAlreadyTaken.into());
                                }
                                // If multiple object in dataset, create a Node object that will
                                // contain our leaves, and set it at given index.
                                // Otherwise lets store the data in a leaf
                                if data_set.len() > 1usize {
                                    // New non leaf containing all generated leaves
                                    node.push_child(HoliumCborNode::NonLeaf(RecursiveNode {
                                        index: Some(explore_index.index),
                                        data: Right(
                                            data_set
                                                .iter()
                                                .enumerate()
                                                .map(|(i, data)| {
                                                    HoliumCborNode::Leaf(ScalarNode {
                                                        index: i as u64,
                                                        data: data.clone(),
                                                    })
                                                })
                                                .collect(),
                                        ),
                                    }));
                                } else {
                                    node.push_child(HoliumCborNode::Leaf(ScalarNode {
                                        index: explore_index.index,
                                        data: data_set
                                            .get(0)
                                            .ok_or(WriteError::NoDataInDataSet)?
                                            .clone(),
                                    }));
                                }
                            }
                            Selector::ExploreIndex(_) | Selector::ExploreRange(_) => {
                                // If node does not exist then create it
                                if !node.has_child(explore_index.index as usize)? {
                                    node.push_child(HoliumCborNode::NonLeaf(RecursiveNode {
                                        index: Some(explore_index.index),
                                        data: Right(vec![]),
                                    }));
                                }

                                return node
                                    .child_as_mut(explore_index.index as usize)?
                                    .ingest(&explore_index.next, data_set);
                            }
                            Selector::ExploreUnion(_) => {
                                return Err(SelectorError::UnionOnlyAtRoot.into())
                            }
                        }
                    }
                    _ => return Err(WriteError::IndexSelectionOnLeaf.into()),
                }
            }
            Selector::ExploreRange(explore_range) => {
                // Making sure that we are on a Node and not a leaf. This can be avoided if selector is
                // properly constructed
                match self {
                    HoliumCborNode::NonLeaf(node) => {
                        // If range then deconstruct in it. But the data set needs to have the exact number of
                        // elements
                        if data_set.len() as u64 != explore_range.end - explore_range.start {
                            return Err(WriteError::DatasetLengthInequalRangeLength.into());
                        }

                        for (i, to_set_index) in
                            (explore_range.start..explore_range.end).enumerate()
                        {
                            // Making sure index is not already taken
                            if node.has_child(to_set_index as usize)? {
                                return Err(WriteError::IndexAlreadyTaken.into());
                            }
                            node.push_child(HoliumCborNode::Leaf(ScalarNode {
                                index: to_set_index,
                                data: data_set[i].clone(),
                            }));
                        }
                    }
                    _ => return Err(WriteError::RangeSelectionOnLeaf.into()),
                }
            }
            Selector::ExploreUnion(_) => return Err(WriteError::UnionOnlyAtRootLevel.into()),
            Selector::Matcher(_) => {
                // If we arrive here it means that we are at root, just checking for safety with
                // an unreachable macro if not on a node type
                match self {
                    HoliumCborNode::NonLeaf(node) => {
                        // If one element then it is our data
                        // Otherwise we build an array out of dataset and use it as data
                        if data_set.len() == 1usize {
                            node.data = Left(data_set[0].clone());
                        } else {
                            let mut buff = generate_array_cbor_header(data_set.len() as u64);
                            for data in data_set.iter_mut() {
                                buff.append(data);
                            }
                            node.data = Left(buff);
                        }
                        return Ok(());
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    /// Generates Cbor object based on the current data held by a [HoliumCborNode] structure
    fn generate_cbor(&self) -> Result<Vec<u8>> {
        match self {
            HoliumCborNode::Leaf(leaf) => Ok(leaf.data.clone()),
            HoliumCborNode::NonLeaf(node) => {
                // Check if root has cbor data instead of children, if so then it is our object
                if node.data.is_left() {
                    return Ok(node
                        .data
                        .as_ref()
                        .left()
                        .ok_or(WriteError::ExpectedCborDataForRecursiveType)?
                        .clone());
                }

                // Otherwise lets create it recursively
                let elements = node
                    .data
                    .as_ref()
                    .right()
                    .ok_or(WriteError::ExpectedChildrenForRecursiveType)?;

                let mut buff: Vec<u8> = generate_array_cbor_header(elements.len() as u64);
                for i in 0..elements.len() {
                    // Find node with index
                    let next_node: Vec<&HoliumCborNode> = elements
                        .iter()
                        .filter(|e| e.get_index().unwrap() == i as u64)
                        .collect::<Vec<&HoliumCborNode>>();

                    buff.append(
                        &mut next_node
                            .get(0)
                            .ok_or(WriteError::NoNodeAtIndex)?
                            .generate_cbor()?,
                    );
                }

                return Ok(buff);
            }
        }
    }
}

/**************************************************
 * Traits
 **************************************************/

pub trait ParseHoliumCbor {
    // To implement to define cursor on reader
    fn as_cursor(&self) -> Cursor<&[u8]>;

    /// Give entire description of a holium cbor serialized object
    fn complete_cbor_structure(&self) -> Result<MajorType> {
        let mut buff = self.as_cursor();

        let mut major_type = read_header(&mut buff)?;

        if !major_type.is_array() {
            return Err(ParseError::RootNotArray.into());
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
            return Err(ParseError::RootNotArray.into());
        }
        read_recursive_elements_detail(&mut buff, &mut major_type).unwrap();

        Ok(major_type.select(&selector_envelope.0)?)
    }

    fn select_cbor(&self, selector_envelope: &SelectorEnvelope) -> Result<Vec<Vec<Vec<u8>>>> {
        let select_cbor_structure = self.select_cbor_structure(selector_envelope)?;

        let mut buff = self.as_cursor();

        retrieve_cbor_in_reader(&mut buff, &select_cbor_structure)
    }
}

pub trait WriteHoliumCbor {
    // To implement to define cursor on reader
    fn as_cursor(&self) -> Cursor<&[u8]>;

    fn copy_cbor<H: ParseHoliumCbor>(
        &self,
        source_data: H,
        tail_selector: &SelectorEnvelope,
        head_selector: &SelectorEnvelope,
    ) -> Result<Vec<u8>> {
        let mut selected_cbor = source_data.select_cbor(tail_selector)?;

        let mut holium_cbor_constructor: HoliumCborNode = HoliumCborNode::NonLeaf(RecursiveNode {
            index: None,
            data: Right(vec![]),
        });
        // If head selector a union, check that tail selector is also one with the same number of
        // selectors
        match &head_selector.0 {
            Selector::ExploreUnion(receiver_union) => match &tail_selector.0 {
                Selector::ExploreUnion(source_union) => {
                    if source_union.0.len() != receiver_union.0.len() {
                        return Err(WriteError::DifferentUnionLength.into());
                    }

                    for (i, data_set) in selected_cbor.iter_mut().enumerate() {
                        holium_cbor_constructor.ingest(&source_union.0[i], data_set)?;
                    }
                }
                _ => return Err(WriteError::NonCompatibleSelectors.into()),
            },
            _ => {
                holium_cbor_constructor.ingest(
                    &head_selector.0,
                    &mut selected_cbor
                        .get_mut(0)
                        .ok_or(WriteError::NoDataInDataSet)?,
                )?;
            }
        }

        Ok(holium_cbor_constructor.generate_cbor()?)
    }
}

/**************************************************
 * Utilities
 **************************************************/

fn get_cursor_position<R: Read + Seek>(reader: &mut R) -> Result<u64> {
    reader
        .stream_position()
        .context(ParseError::FailToGetCursorPosition)
}
/// Read returning its major type, its data byte offset and size
fn read_header<R: Read + Seek>(reader: &mut R) -> Result<MajorType> {
    // Save he&der offset for later use
    let header_offset = get_cursor_position(reader)?;

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
        0 | 1 => match data_details {
            0..=23 => Ok((Some(header_offset), 1)),
            24 => Ok((Some(header_offset + 1), 1)),
            25 => Ok((Some(header_offset + 1), 2)),
            26 => Ok((Some(header_offset + 1), 4)),
            27 => Ok((Some(header_offset + 1), 8)),
            _ => return Err(ParseError::UnhandledDataDetails.into()),
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
                _ => return Err(ParseError::UnhandledDataDetails.into()),
            };

            // Get current offset
            let complementary_bytes_offset = header_offset + 1;

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
                .context(ParseError::FailToSeekToOffset)?;

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
                    .context(ParseError::FailToSeekToOffset)?;
            }
        }
        _ => return Err(ParseError::MajorTypeNonRecursive.into()),
    }

    Ok(())
}

/// [retrieve_cbor_in_reader] will fetch cbor data in reader based on a [MajorType] structure containing
/// structural information of wherethe data is in the reader
fn retrieve_cbor_in_reader<R: Read + Seek>(
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
fn generate_array_cbor_header(size: u64) -> Vec<u8> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::local::models::data::HoliumCbor;
    use serde_cbor::Value as CborValue;
    use serde_json::{json, Value as JsonValue};
    use std::collections::BTreeMap;

    pub trait Importable {
        /// Convert an object to a valid CBOR value.
        fn to_cbor(&self) -> CborValue;
    }

    impl Importable for JsonValue {
        fn to_cbor(&self) -> CborValue {
            match self {
                JsonValue::Null => CborValue::Null,
                JsonValue::Bool(v) => CborValue::Bool(*v),
                JsonValue::Number(v) if { v.is_i64() } => {
                    CborValue::Integer(v.as_i64().unwrap() as i128)
                }
                JsonValue::Number(v) if { v.is_u64() } => {
                    CborValue::Integer(v.as_u64().unwrap() as i128)
                }
                JsonValue::Number(v) if { v.is_f64() } => CborValue::Float(v.as_f64().unwrap()),
                JsonValue::String(v) => CborValue::Text(v.to_string()),
                JsonValue::Array(v) => {
                    CborValue::Array(v.iter().map(|v| v.to_owned().to_cbor()).collect())
                }
                JsonValue::Object(v) => {
                    let mut cbor_map: BTreeMap<CborValue, CborValue> = BTreeMap::new();
                    for (key, value) in v {
                        cbor_map.insert(CborValue::Text(key.to_string()), value.to_cbor());
                    }
                    CborValue::Map(cbor_map)
                }
                JsonValue::Number(_) => unreachable!(),
            }
        }
    }
    #[test]
    fn testing() {
        let json_value: JsonValue = json!([0, 1, 2]);
        let cbor_value = json_value.to_cbor();
        let cbor: HoliumCbor = to_vec(&cbor_value).unwrap();

        let matcher_selector = "{\".\": {}}";
        let index_selector = "{\"i\": {\"i\": 1, \">\": { \".\": {}}}}";
        let range_selector = "{\"r\": {\"^\": 1,\"$\": 3,\">\": {\".\": {}}}}";

        let tail_selector = SelectorEnvelope::new(index_selector).unwrap();
        let head_selector = SelectorEnvelope::new(matcher_selector).unwrap();

        let mut new_data: HoliumCbor = Vec::new();

        dbg!(new_data
            .copy_cbor(cbor, &tail_selector, &head_selector)
            .unwrap());
    }
}
