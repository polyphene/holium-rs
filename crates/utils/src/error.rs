use crate::tree::NodeIndex;

/// TreeError represents all errors that might happen around `HoliumTree` handling
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum HoliumTreeError {
    #[error("new node should not have children")]
    NewNodeNoChildrenError,
    #[error("node not found in tree")]
    NodeNotFound(NodeIndex),
    #[error("wrong node type")]
    WrongNodeTypeError(NodeIndex),
    #[error("can not remove root from tree")]
    RootNoRemovalError,
}
