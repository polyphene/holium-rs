//! Define traits and structure necessary to implement a Tree in the Holium Framework

use crate::error::HoliumTreeError;

/*************************************************************
 * Update Trait
 *************************************************************/

/// `HoliumTreeData` is a trait that will determine data behaviour in the tree based on manipulations that
/// are applied.
///
/// `Ld` is the data structure implemented in the tree to be used as a data value for the tree's leaves.
/// `Nd` is the data structure implemented in the tree to be used as a data value for the tree's nodes.
pub trait HoliumTreeData<Ld, Nd = Ld>
where
    Ld: Clone,
    Nd: Clone,
{
    /// Function called when a node child is added
    fn on_new_child(&mut self, children: Vec<HoliumNode<Ld, Nd>>);
    /// Function called when a node child is updated
    fn on_child_updated(&mut self, children: Vec<HoliumNode<Ld, Nd>>);
    /// Function called when a node child is removed
    fn on_child_removed(&mut self, children: Vec<HoliumNode<Ld, Nd>>);
}

/// `HoliumTreeEvents` is an internal enum used on recursive bottom up pathing to know which functions
/// of the `HoliumTreeData` has to be called on the node
#[derive(Debug, Clone, Eq, PartialEq)]
enum HoliumTreeEvents {
    // Variant used to trigger a new child event on some node data
    NewChildEvent,
    // Variant used to trigger an updated child event on some node data
    ChildUpdatedEvent,
    // Variant used to trigger a removed child event on some node data
    ChildRemovedEvent,
}

/*************************************************************
 * Tree
 *************************************************************/
/// `NodeIndex` is the index of the node inside the flat node list that is composing our tree.
pub(crate) type NodeIndex = usize;

/// `HoliumTree` is a generic tree structure that holds generic data type in its nodes and leaves.
/// The tree is composed of a flat node list, `nodes`. The nodes point to their children and for
/// easier computation children to their parent.
///
/// `Ld` is a generic structure that will be used as a data value for the leaves in our tree.
/// `Nd` is a generic structure that will be used as a data value for the nodes in our tree. It has to
/// implement the [`HoliumTreeData<Ld, Nd>`] trait.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HoliumTree<Ld, Nd = Ld>
where
    Ld: Clone,
    Nd: Clone,
{
    nodes: Vec<HoliumNode<Ld, Nd>>, // Tree's nodes list
}

impl<Ld: Clone, Nd: Clone> HoliumTree<Ld, Nd>
where
    Ld: Clone,
    Nd: Clone + HoliumTreeData<Ld, Nd>,
{
    /**************************************
     * Initializer
     **************************************/

    pub fn new(root_type: HoliumNodeType<Ld, Nd>) -> Result<Self, HoliumTreeError> {
        let root = HoliumNode::root(root_type)?;

        Ok(HoliumTree { nodes: vec![root] })
    }

    /**************************************
     * Getter
     **************************************/

    pub fn nodes(&self) -> &[HoliumNode<Ld, Nd>] {
        &self.nodes
    }

    pub fn node(&self, node_index: NodeIndex) -> Option<&HoliumNode<Ld, Nd>> {
        self.nodes.get(node_index)
    }

    pub fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    /// `children` returns the children of a given `HoliumNode`. Will return `None` if either the node
    /// type is a leaf or if the node has no children.
    pub fn children(&self, node_index: NodeIndex) -> Option<Vec<HoliumNode<Ld, Nd>>> {
        self.nodes.get(node_index)?;

        if self.nodes[node_index].node_type.is_leaf()
            || (self.nodes[node_index].node_type.is_node()
                && !self.nodes[node_index].node_type.has_children())
        {
            return None;
        }

        Some(
            self.nodes
                .iter()
                .filter(|n| n.parent.is_some() && n.parent.unwrap() == node_index)
                .map(|n| n.clone())
                .collect(),
        )
    }

    /// `children_references` same as [`HoliumTree::children(&self, node_index: NodeIndex)`] but with
    /// children as references
    pub fn children_references(&self, node_index: NodeIndex) -> Vec<&HoliumNode<Ld, Nd>> {
        self.nodes
            .iter()
            .filter(|n| n.parent.is_some() && n.parent.unwrap() == node_index)
            .collect()
    }

    /**************************************
     * Setter
     **************************************/
    /// `add_leaf` will add a new leaf to the `HoliumTree`. The parent as to be  [`HoliumNodeType::Node`].
    /// Will trigger the trait [`HoliumTreeData::on_new_child()`] for its parent and [`HoliumTreeData::on_child_updated()`]
    /// for subsequent parent nodes.
    pub fn add_leaf(
        &mut self,
        parent_index: NodeIndex,
        data: Ld,
    ) -> Result<&mut Self, HoliumTreeError> {
        if self.nodes.get(parent_index).is_none() {
            return Err(HoliumTreeError::NodeNotFound(parent_index));
        }

        if self.nodes[parent_index].node_type.is_leaf() {
            return Err(HoliumTreeError::WrongNodeTypeError(parent_index));
        }

        // First we add new node to tree
        let leaf: HoliumNode<Ld, Nd> =
            HoliumNode::new(self.nodes_len(), parent_index, HoliumNodeType::Leaf(data))?;
        self.nodes.push(leaf.clone());
        self.nodes[parent_index].new_child(leaf.index);

        let event = HoliumTreeEvents::NewChildEvent;

        self.bottom_up_recursive_pathing(parent_index, event);

        Ok(self)
    }

    /// `add_node` will add a new node to the `HoliumTree`. The parent as to be  [`HoliumNodeType::Node`].
    /// Will trigger the trait [`HoliumTreeData::on_new_child()`] for its parent and [`HoliumTreeData::on_child_updated()`]
    /// for subsequent parent nodes.
    pub fn add_node(
        &mut self,
        parent_index: NodeIndex,
        data: Nd,
    ) -> Result<&mut Self, HoliumTreeError> {
        if self.nodes.get(parent_index).is_none() {
            return Err(HoliumTreeError::NodeNotFound(parent_index));
        }

        if self.nodes[parent_index].node_type.is_leaf() {
            return Err(HoliumTreeError::WrongNodeTypeError(parent_index));
        }

        // First we add new node to tree
        let node: HoliumNode<Ld, Nd> = HoliumNode::new(
            self.nodes_len(),
            parent_index,
            HoliumNodeType::Node((data, vec![])),
        )?;
        self.nodes.push(node.clone());
        self.nodes[parent_index].new_child(node.index);

        let event = HoliumTreeEvents::NewChildEvent;

        self.bottom_up_recursive_pathing(parent_index, event);

        Ok(self)
    }

    /// `remove_leaf` will remove a leaf from the `HoliumTree`. Will trigger the trait
    /// [`HoliumTreeData::on_child_removed()`] for its parent and [`HoliumTreeData::on_child_updated()`]
    /// for subsequent parent nodes.
    pub fn remove_leaf(&mut self, leaf_index: NodeIndex) -> Result<&mut Self, HoliumTreeError> {
        if leaf_index == 0 {
            return Err(HoliumTreeError::RootNoRemovalError);
        }

        if self.nodes.get(leaf_index).is_none() {
            return Err(HoliumTreeError::NodeNotFound(leaf_index));
        }

        if self.nodes[leaf_index].node_type.is_node() {
            return Err(HoliumTreeError::WrongNodeTypeError(leaf_index));
        }

        let parent_index = self.nodes[leaf_index].parent.unwrap();

        self.recursive_retain(parent_index, leaf_index);
        self.sanitize_indexes();

        let event = HoliumTreeEvents::ChildRemovedEvent;

        self.bottom_up_recursive_pathing(parent_index, event);

        Ok(self)
    }

    /// `remove_node` will remove a node and its children from the `HoliumTree`. Will trigger the trait
    /// [`HoliumTreeData::on_child_removed()`] for its parent and [`HoliumTreeData::on_child_updated()`]
    /// for subsequent parent nodes.
    pub fn remove_node(&mut self, node_index: NodeIndex) -> Result<&mut Self, HoliumTreeError> {
        if node_index == 0 {
            return Err(HoliumTreeError::RootNoRemovalError);
        }

        if self.nodes.get(node_index).is_none() {
            return Err(HoliumTreeError::NodeNotFound(node_index));
        }

        if self.nodes[node_index].node_type.is_leaf() {
            return Err(HoliumTreeError::WrongNodeTypeError(node_index));
        }

        let parent_index = self.nodes[node_index].parent.unwrap();

        self.recursive_retain(parent_index, node_index);

        let event = HoliumTreeEvents::ChildRemovedEvent;

        self.bottom_up_recursive_pathing(parent_index, event);

        Ok(self)
    }

    /// `update_leaf_data` will update the data of a leaf in the `HoliumTree`. Will trigger the trait
    /// [`HoliumTreeData::on_child_updated()`] for its parent and subsequent parent nodes.
    pub fn update_leaf_data(
        &mut self,
        leaf_index: NodeIndex,
        leaf_data: Ld,
    ) -> Result<&mut Self, HoliumTreeError> {
        if self.nodes.get(leaf_index).is_none() {
            return Err(HoliumTreeError::NodeNotFound(leaf_index));
        }

        if self.nodes[leaf_index].node_type.is_node() {
            return Err(HoliumTreeError::WrongNodeTypeError(leaf_index));
        }

        let parent_index = self.nodes[leaf_index].parent.unwrap();

        let current_data = self.nodes[leaf_index].leaf_data_mut().unwrap();
        *current_data = leaf_data;

        let event = HoliumTreeEvents::ChildUpdatedEvent;

        self.bottom_up_recursive_pathing(parent_index, event);

        Ok(self)
    }

    /**************************************
     * Utilities
     **************************************/

    // Internal function, recursively travel in a bottom-up fashioon in the Tree, calling for nodes
    // updates.
    fn bottom_up_recursive_pathing(
        &mut self,
        node_index: NodeIndex,
        event: HoliumTreeEvents,
    ) -> &mut Self {
        let children: Vec<HoliumNode<Ld, Nd>> = match self.children(node_index) {
            Some(children) => children,
            None => vec![],
        };

        self.trigger_node_update(node_index, event, children);
        // Node is root
        if node_index == 0 {
            return self;
        }

        self.bottom_up_recursive_pathing(
            self.nodes[node_index].parent.unwrap(),
            HoliumTreeEvents::ChildUpdatedEvent,
        )
    }

    // Internal function, in charge of calling the [`HoliumTreeData`] trait when the tree is updated.
    fn trigger_node_update(
        &mut self,
        node_index: NodeIndex,
        event: HoliumTreeEvents,
        children: Vec<HoliumNode<Ld, Nd>>,
    ) -> &mut Self {
        match event {
            HoliumTreeEvents::NewChildEvent => self.nodes[node_index]
                .node_data_mut()
                .unwrap()
                .on_new_child(children),
            HoliumTreeEvents::ChildUpdatedEvent => self.nodes[node_index]
                .node_data_mut()
                .unwrap()
                .on_child_updated(children),
            HoliumTreeEvents::ChildRemovedEvent => self.nodes[node_index]
                .node_data_mut()
                .unwrap()
                .on_child_removed(children),
        };
        self
    }

    // Internal function, use to recursively delete nodes when a node type is removed from the tree.
    fn recursive_retain(&mut self, parent_index: NodeIndex, node_index: NodeIndex) -> &mut Self {
        if self.nodes[node_index].has_children() {
            let children = self.children(node_index).unwrap();
            let children_indexes: Vec<NodeIndex> = children.iter().map(|n| n.index).collect();

            for index in children_indexes {
                self.recursive_retain(node_index, index);
            }
        }

        self.nodes[parent_index].retain_child(node_index);
        self.nodes.retain(|n| n.index != node_index);

        self
    }

    // Internal function, making sure the node indexes of the tree are up to date.
    fn sanitize_indexes(&mut self) -> &mut Self {
        let iter = std::iter::IntoIterator::into_iter(&mut self.nodes);
        iter.enumerate().for_each(|(i, n)| n.index = i);

        self
    }
}

/*************************************************************
 * Tree Node
 *************************************************************/
/// `HoliumNode` represents all nodes inside an `HoliumTree`. The `node_type` attributes determine if
/// it is a leaf or node.
///
/// `Ld` is the data structure implemented in the tree to be used as a data value for the tree's leaves.
/// `Nd` is the data structure implemented in the tree to be used as a data value for the tree's nodes.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HoliumNode<Ld, Nd = Ld>
where
    Ld: Clone,
    Nd: Clone,
{
    index: NodeIndex,
    parent: Option<NodeIndex>, // Index of parent node, if none is root
    node_type: HoliumNodeType<Ld, Nd>, // Type of the node, leaf or node
}

impl<Ld, Nd> HoliumNode<Ld, Nd>
where
    Ld: Clone,
    Nd: Clone,
{
    /**************************************
     * Initializer
     **************************************/

    fn new(
        index: NodeIndex,
        parent_index: NodeIndex,
        node_type: HoliumNodeType<Ld, Nd>,
    ) -> Result<Self, HoliumTreeError> {
        if node_type.is_node() && node_type.has_children() {
            return Err(HoliumTreeError::NewNodeNoChildrenError);
        }

        Ok(HoliumNode {
            index,
            parent: Some(parent_index),
            node_type,
        })
    }

    /// Internal function used to generate the root of the tree.
    fn root(root_type: HoliumNodeType<Ld, Nd>) -> Result<Self, HoliumTreeError> {
        if root_type.is_node() && root_type.has_children() {
            return Err(HoliumTreeError::NewNodeNoChildrenError);
        }

        Ok(HoliumNode {
            index: 0,
            parent: None,
            node_type: root_type,
        })
    }

    /**************************************
     * Getter
     **************************************/

    pub fn index(&self) -> NodeIndex {
        self.index
    }

    pub fn parent(&self) -> Option<NodeIndex> {
        self.parent
    }

    pub fn node_type(&self) -> &HoliumNodeType<Ld, Nd> {
        &self.node_type
    }

    fn node_data_mut(&mut self) -> Option<&mut Nd> {
        match &mut self.node_type {
            HoliumNodeType::Node((data, _)) => Some(data),
            _ => None,
        }
    }

    fn leaf_data_mut(&mut self) -> Option<&mut Ld> {
        match &mut self.node_type {
            HoliumNodeType::Leaf(data) => Some(data),
            _ => None,
        }
    }

    /**************************************
     * Setter for nodes
     **************************************/
    fn new_child(&mut self, child_index: NodeIndex) -> Option<Vec<NodeIndex>> {
        self.node_type.new_child(child_index)
    }

    fn retain_child(&mut self, child_index: NodeIndex) -> Option<Vec<NodeIndex>> {
        self.node_type.retain_child(child_index)
    }

    /**************************************
     * Utilities
     **************************************/
    fn has_children(&self) -> bool {
        self.node_type.has_children()
    }
}

/// `HoliumNodeType` is an enum to identify a node type.
/// A `Leaf` will only contain some data wile a `Node` will contain its data and an ordered list of
/// all its children.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum HoliumNodeType<Ld, Nd = Ld>
where
    Ld: Clone,
    Nd: Clone,
{
    Leaf(Ld),
    Node((Nd, Vec<NodeIndex>)),
}

impl<Ld: Clone, Nd: Clone> HoliumNodeType<Ld, Nd> {
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

    pub fn has_children(&self) -> bool {
        match self {
            Self::Node((_, children)) => children.len() > 0,
            _ => false,
        }
    }

    pub fn new_child(&mut self, child_index: NodeIndex) -> Option<Vec<NodeIndex>> {
        match self {
            Self::Node((_, children)) => {
                children.push(child_index);
                Some(children.clone())
            }
            _ => None,
        }
    }

    pub fn retain_child(&mut self, child_index: NodeIndex) -> Option<Vec<NodeIndex>> {
        match self {
            Self::Node((_, children)) => {
                if !children.contains(&child_index) {
                    return None;
                }

                children.retain(|c| *c != child_index);
                Some(children.clone())
            }
            _ => None,
        }
    }
}
