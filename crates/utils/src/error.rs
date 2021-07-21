use crate::tree::NodeIndex;

/// TreeError represents all errors that might happened around `HoliumTree` handling
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum HoliumTreeError {
    #[error("Parent not found in tree")]
    ParentNotFoundError(NodeIndex),
    #[error("Parent node should not be a leaf")]
    WrongParentTypeError(NodeIndex),
    #[error("Leaf has a child")]
    LeafIsParentError(NodeIndex),
}
