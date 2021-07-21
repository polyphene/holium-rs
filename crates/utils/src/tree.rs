use crate::error::HoliumTreeError;
use std::borrow::Borrow;
use std::ops::Deref;

/*************************************************************
 * Update Trait
 *************************************************************/

/// TreeData is a trait that will determine data behaviour in the tree based on manipulations that
/// are applied.
pub trait HoliumTreeData<Ld: Clone, Nd = Ld>
where
    Ld: Clone,
    Nd: Clone,
{
    /// Function when a node child is added
    fn on_new_child(&mut self, child: &HoliumNode<Ld, Nd>) -> &mut Self;
    /// Function called when a node child is updated
    fn on_child_update(&mut self, child: &HoliumNode<Ld, Nd>) -> &mut Self;
    /// Function called when a node child is removed
    fn on_child_removed(&mut self, child: &HoliumNode<Ld, Nd>) -> &mut Self;
}

/*************************************************************
 * Tree
 *************************************************************/

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoliumTree<Ld, Nd = Ld>
where
    Ld: Clone,
    Nd: Clone,
{
    pub nodes: Vec<HoliumNode<Ld, Nd>>, // Tree's nodes list
}

impl<Ld, Nd> HoliumTree<Ld, Nd>
where
    Ld: Clone,
    Nd: Clone + HoliumTreeData<Ld, Nd>,
{
    pub fn new(root_type: HoliumNodeType<Ld, Nd>) -> Self {
        HoliumTree {
            nodes: vec![HoliumNode::root(root_type)],
        }
    }

    fn push_node(&mut self, node: HoliumNode<Ld, Nd>) -> &mut Self {
        self.nodes.push(node);

        self
    }

    pub fn add_leaf(
        &mut self,
        parent_index: NodeIndex,
        data: Ld,
    ) -> Result<&mut Self, HoliumTreeError> {
        let leaf_index = self.nodes.deref().len() as u32 - 1;
        if leaf_index < parent_index {
            return Err(HoliumTreeError::ParentNotFoundError(parent_index));
        }

        let parent = &mut self.nodes[parent_index as usize];

        if parent.node_type.is_leaf() {
            return Err(HoliumTreeError::WrongParentTypeError(parent_index));
        }

        // First we add new node to tree
        let leaf: HoliumNode<Ld, Nd> =
            HoliumNode::new(leaf_index, parent_index, HoliumNodeType::Leaf(data));
        self.nodes.push(leaf.clone());

        // Then we are going up the tree to call for data updates on parent
        let mut current_node: &mut HoliumNode<Ld, Nd> = parent;
        let mut child: &HoliumNode<Ld, Nd> = &leaf;
        loop {
            let data_mut = current_node.node_data_mut().unwrap();

            data_mut.on_new_child(child);

            if current_node.parent.is_none() {
                break;
            } else {
                if self.nodes[current_node.parent.unwrap() as usize]
                    .node_type
                    .is_leaf()
                {
                    return Err(HoliumTreeError::LeafIsParentError(current_node.index));
                }

                child = current_node.deref();
                current_node = &mut self.nodes[current_node.parent.unwrap() as usize];
            }
        }

        Ok(self)
    }

    pub fn add_node(&mut self) -> Result<&mut Self, HoliumTreeError> {
        // TODO: implement node addition, node

        Ok(self)
    }
}

/*************************************************************
 * Tree Node
 *************************************************************/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoliumNode<Ld, Nd = Ld>
where
    Ld: Clone,
    Nd: Clone,
{
    index: NodeIndex,
    parent: Option<NodeIndex>, // Index of parent node, if none is root
    node_type: HoliumNodeType<Ld, Nd>, // Type of the node, leaf or node
}

impl<Ld: Clone, Nd: Clone> HoliumNode<Ld, Nd> {
    pub fn new(
        index: NodeIndex,
        parent_index: NodeIndex,
        node_type: HoliumNodeType<Ld, Nd>,
    ) -> Self {
        let mut node = HoliumNode {
            index,
            parent: Some(parent_index),
            node_type,
        };

        node
    }

    pub(crate) fn root(root_type: HoliumNodeType<Ld, Nd>) -> Self {
        HoliumNode {
            index: 0,
            parent: None,
            node_type: root_type,
        }
    }

    pub fn node_data_mut(&mut self) -> Option<&mut Nd> {
        match &mut self.node_type {
            HoliumNodeType::Node((data, _)) => Some(data),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HoliumNodeType<Ld, Nd = Ld> {
    Leaf(Ld),
    Node((Nd, Vec<NodeIndex>)),
}

impl<Ld, Nd> HoliumNodeType<Ld, Nd> {
    pub fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf(_) => true,
            _ => false,
        }
    }

    pub fn is_node(&self) -> bool {
        match self {
            Self::Node(_) => true,
            _ => false,
        }
    }
}

pub(crate) type NodeIndex = u32;
