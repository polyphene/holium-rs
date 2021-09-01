//! Module listing common errors used throughout the crate.
use thiserror::Error;

#[derive(Error, Debug)]
/// Type for CLI common errors
pub(crate) enum CommonError {
    /// Thrown when a provided identifier cannot be linked to a known Holium object
    #[error("unknown object identifier: {0}")]
    UnknownObjectIdentifier(String),
}