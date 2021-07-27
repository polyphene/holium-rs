use crate::tree::NodeIndex;

/// TreeError represents all errors that might happened around `HoliumTree` handling
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum HoliumTreeError {
    #[error("New Node should not have children")]
    NewNodeNoChildrenError,
    #[error("Node not found in tree")]
    NodeNotFound(NodeIndex),
    #[error("Wrong node type")]
    WrongNodeTypeError(NodeIndex),
    #[error("Can not remove root from tree")]
    RootNoRemovalError,
}
