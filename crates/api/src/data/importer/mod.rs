//! Methods related to the import of holium data objects from multiple file formats.

use serde_cbor::Value as CborValue;

pub mod bin;
pub mod cbor;
pub mod csv;
pub mod json;

/// This trait is shared by types tha can be imported as holium data in the Holium Framework.
/// Any type which objects can be converted into a CBOR object may implement this trait.
pub trait Importable {
    /// Convert an object to a valid CBOR value.
    fn to_cbor(&self) -> CborValue;
}
