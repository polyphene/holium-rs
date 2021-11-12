//! This module defines a trait and implementations helping serializing different objects of the
//! Holium framework into formats, often fragmented, suitable for storage.

use anyhow::Result;
use cid::Cid;
use std::io::Read;
use thiserror::Error;

mod linked_data_tree;
mod transformation;

#[derive(Debug, Error)]
/// Errors for the [fragment_serialize] module.
enum FragmentSerializeError {
    /// This error is thrown when an unreachable line of code is reached in case the [is_of_type] method
    /// has not proven that [value_from_bytes] could safely be executed.
    #[error("wrong deserializer has been executed on bytes when trying to deserialize a fragment")]
    WrongDeserializer,
}

/// [FragmentedDataDeserResult] is the format of objects resulting from the parsing of aa serialized
/// object representing an element of an object from the Holium Framework, in its fragmented state.
pub struct FragmentedDataDeserResult<T> {
    /// The [value] field should hold an object in a type usable in the Holium Framework.
    value: T,
    /// The [links] field should include CIDs of all other fragments that are linked in the parsed object,
    /// and which retrieval and parsing are also necessary for recursive parsing of the unified object.
    /// Uniqueness of CIDs in this list is not considered mandatory.
    links: Vec<Cid>,
}

/// The [HoliumDeserializable] trait should be implemented for types representing fragments of data in
/// the Holium Framework and that may in that sense be serialized for efficient storage.
pub trait HoliumDeserializable {
    /// [is_of_type] checks if an array of bytes could be deserialized into the given Self type.
    fn is_of_type<R: Read>(data_reader: &mut R) -> Result<bool>;
    /// [value_from_bytes] deserializes an object of Self type.
    /// [is_of_type] should always be run before trying to deserialize an object.
    /// If an array of byte can be deserialized into an object of Self type according to the [is_of_type]
    /// function, then [value_from_bytes] is guaranteed to return the deserialized object.
    ///
    /// # Panics
    ///
    /// This method may panic in case the array of bytes cannot be deserialized into Self.
    /// For this reason, the result of [is_of_type] should always be checked before running this method.
    fn value_from_bytes(data: &[u8]) -> FragmentedDataDeserResult<Box<Self>>;
}
