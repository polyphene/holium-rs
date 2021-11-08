use crate::utils::cbor::as_holium_cbor::AsHoliumCbor;
use crate::utils::cbor::helpers::{generate_array_cbor_header, SelectorError, WriteError};
use crate::utils::interplanetary::kinds::selector::{Selector, SelectorEnvelope};
use anyhow::Result;
use either::Either;
use either::Either::{Left, Right};
use std::borrow::Borrow;
use std::io::Cursor;

pub trait WriteHoliumCbor {
    // To implement to define cursor on reader
    fn as_cursor(&self) -> Cursor<&[u8]>;

    fn copy_cbor<H: AsHoliumCbor>(
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
