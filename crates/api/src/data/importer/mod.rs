//! Methods related to the import of holium data objects fro multiple file formats.

mod json;
mod cbor;

use serde_cbor::Value as CborValue;

/// This trait is shared by types tha can be imported as holium data in the Holium Framework.
/// Any type which objects can be converted into a CBOR object may implement this trait.
pub(crate) trait Importable {
    /// Convert an object to a valid CBOR value.
    fn to_cbor(self) -> CborValue;
}