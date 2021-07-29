//! Define errors for the utilities crate.

use crate::tree::NodeIndex;

/// TreeError represents all errors that might happen around `Tree` handling
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum TreeError {
    #[error("new node should not have children")]
    NewNodeNoChildrenError,
    #[error("node not found in tree")]
    NodeNotFound(NodeIndex),
    #[error("wrong node type")]
    WrongNodeTypeError(NodeIndex),
    #[error("can not remove root from tree")]
    RootNoRemovalError,
}
