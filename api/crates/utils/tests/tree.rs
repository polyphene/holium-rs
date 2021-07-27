use holium_utils::error::HoliumTreeError;
use holium_utils::tree::{HoliumNode, HoliumNodeType, HoliumTree, HoliumTreeData};

/**************************************
 * Init data types for tree
 **************************************/
#[derive(Clone, Debug, Eq, PartialEq)]
struct Ld {
    attr: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Nd {
    attr: u32,
}

impl HoliumTreeData<Ld, Nd> for Nd {
    fn on_new_child(&mut self, children: Vec<HoliumNode<Ld, Nd>>) {
        let mut value: u32 = 0;

        for (_, node) in children.iter().enumerate() {
            value += match node.node_type() {
                HoliumNodeType::Leaf(data) => data.attr,
                HoliumNodeType::Node((data, _)) => data.attr,
            };
        }
        self.attr = value;
    }

    fn on_child_updated(&mut self, children: Vec<HoliumNode<Ld, Nd>>) {
        let mut value: u32 = 0;

        for (_, node) in children.iter().enumerate() {
            value += match node.node_type() {
                HoliumNodeType::Leaf(data) => data.attr,
                HoliumNodeType::Node((data, _)) => data.attr,
            };
        }
        self.attr = value;
    }

    fn on_child_removed(&mut self, children: Vec<HoliumNode<Ld, Nd>>) {
        let mut value: u32 = 0;

        for (_, node) in children.iter().enumerate() {
            value += match node.node_type() {
                HoliumNodeType::Leaf(data) => data.attr,
                HoliumNodeType::Node((data, _)) => data.attr,
            };
        }
        self.attr = value;
    }
}

/**************************************
 * Test Tree
 **************************************/
#[test]
fn test_new_tree() {
    /**************************************
     * Test new tree, root is leaf
     **************************************/
    let leaf_data = Ld { attr: 45 };

    let node_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Leaf(leaf_data);

    let result: Result<HoliumTree<Ld, Nd>, HoliumTreeError> = HoliumTree::new(node_type.clone());

    assert_eq!(true, result.is_ok());

    let tree = result.unwrap();
    assert_eq!(1, tree.nodes().len());

    let root: &HoliumNode<Ld, Nd> = tree.node(0).unwrap();
    assert_eq!(0, root.index());
    assert_eq!(None, root.parent());
    assert_eq!(&node_type, root.node_type());

    /**************************************
     * Test new tree, root is node
     **************************************/
    let node_data = Nd { attr: 45 };

    // Node type is given children
    let node_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((node_data.clone(), vec![1]));

    let result: Result<HoliumTree<Ld, Nd>, HoliumTreeError> = HoliumTree::new(node_type.clone());

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::NewNodeNoChildrenError,
        result.err().unwrap()
    );

    // Node type is properly initialized
    let node_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((node_data, vec![]));

    let result: Result<HoliumTree<Ld, Nd>, HoliumTreeError> = HoliumTree::new(node_type.clone());

    assert_eq!(true, result.is_ok());

    let tree = result.unwrap();
    assert_eq!(1, tree.nodes().len());

    let root: &HoliumNode<Ld, Nd> = tree.node(0).unwrap();
    assert_eq!(0, root.index());
    assert_eq!(None, root.parent());
    assert_eq!(&node_type, root.node_type());
}

#[test]
fn test_add_new_leaf() {
    /**************************************
     * Fails if tree has leaf root
     **************************************/
    let root_data = Ld { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Leaf(root_data);

    let mut tree = HoliumTree::new(root_type).unwrap();

    let leaf_data = Ld { attr: 10 };

    let parent_index = 0;
    let result = tree.add_leaf(parent_index, leaf_data);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::WrongNodeTypeError(parent_index),
        result.err().unwrap()
    );

    /**************************************
     * New tree with correct root
     **************************************/
    let root_data = Nd { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((root_data, vec![]));

    let mut tree = HoliumTree::new(root_type).unwrap();

    /**************************************
     * Fails if parent index does not exists
     **************************************/
    let leaf_data = Ld { attr: 10 };

    let parent_index = 3;
    let result = tree.add_leaf(parent_index, leaf_data);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::NodeNotFound(parent_index),
        result.err().unwrap()
    );

    /**************************************
     * Success
     **************************************/
    let leaf_data = Ld { attr: 10 };

    let parent_index = 0;
    let result = tree.add_leaf(parent_index, leaf_data.clone());

    assert_eq!(true, result.is_ok());

    let tree = result.unwrap();

    let root_data: &Nd = match tree.node(0).unwrap().node_type() {
        HoliumNodeType::Node((data, _)) => data,
        _ => {
            panic!("root is supposed to be Node here")
        }
    };

    let leaf_type_should_be = HoliumNodeType::Leaf(leaf_data);

    // Checking that there has been an addition in nodes
    assert_eq!(2, tree.nodes_len());

    // Making sure that root children have been updated
    let result_root_children = tree.children(0);
    assert_eq!(true, result_root_children.is_some());
    let root_children: Vec<HoliumNode<Ld, Nd>> = result_root_children.unwrap();
    assert_eq!(1, root_children.len());

    // Check that node is properly formed
    let root_child = &root_children[0];
    assert_eq!(1, root_child.index());
    assert_eq!(true, root_child.parent().is_some());
    assert_eq!(0, root_child.parent().unwrap());
    assert_eq!(&leaf_type_should_be, root_child.node_type());

    // Check that root has been updated par HoliumTreeData trait
    assert_eq!(10, root_data.attr);
}

#[test]
fn test_add_new_node() {
    /**************************************
     * Fails if tree has leaf root
     **************************************/
    let root_data = Ld { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Leaf(root_data);

    let mut tree = HoliumTree::new(root_type).unwrap();

    let node_data = Nd { attr: 10 };

    let parent_index = 0;
    let result = tree.add_node(parent_index, node_data);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::WrongNodeTypeError(parent_index),
        result.err().unwrap()
    );

    /**************************************
     * New tree with correct root
     **************************************/
    let root_data = Nd { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((root_data, vec![]));

    let mut tree = HoliumTree::new(root_type).unwrap();

    /**************************************
     * Fails if parent index does not exists
     **************************************/
    let node_data = Nd { attr: 10 };

    let parent_index = 3;
    let result = tree.add_node(parent_index, node_data);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::NodeNotFound(parent_index),
        result.err().unwrap()
    );

    /**************************************
     * Success
     **************************************/
    let node_data = Nd { attr: 10 };

    let parent_index = 0;
    let result = tree.add_node(parent_index, node_data.clone());

    assert_eq!(true, result.is_ok());

    let tree = result.unwrap();
    let root_data: &Nd = match tree.node(0).unwrap().node_type() {
        HoliumNodeType::Node((data, _)) => data,
        _ => {
            panic!("root is supposed to be Node here")
        }
    };

    let node_type_should_be: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((node_data, vec![]));

    // Checking that there has been an addition in nodes
    assert_eq!(2, tree.nodes_len());

    // Making sure that root children have been updated
    let result_root_children = tree.children(0);
    assert_eq!(true, result_root_children.is_some());
    let root_children: Vec<HoliumNode<Ld, Nd>> = result_root_children.unwrap();
    assert_eq!(1, root_children.len());

    // Check that node is properly formed
    let root_child = &root_children[0];
    assert_eq!(1, root_child.index());
    assert_eq!(true, root_child.parent().is_some());
    assert_eq!(0, root_child.parent().unwrap());
    assert_eq!(&node_type_should_be, root_child.node_type());

    // Check that root has been updated par HoliumTreeData trait
    assert_eq!(10, root_data.attr);
}

#[test]
fn test_remove_leaf() {
    /**************************************
     * Fails if tree root is leaf
     **************************************/
    let root_data = Ld { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Leaf(root_data);

    let mut tree = HoliumTree::new(root_type).unwrap();

    let result = tree.remove_leaf(0);

    assert_eq!(true, result.is_err());
    assert_eq!(HoliumTreeError::RootNoRemovalError, result.err().unwrap());

    /**************************************
     * New tree and new leaf
     **************************************/
    let root_data = Nd { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((root_data, vec![]));

    let mut tree = HoliumTree::new(root_type).unwrap();

    let leaf_data = Ld { attr: 10 };

    let parent_index = 0;
    let _ = tree.add_leaf(parent_index, leaf_data.clone());

    /**************************************
     * Fails if node does not exists
     **************************************/
    let leaf_index = 10;
    let result = tree.remove_leaf(leaf_index);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::NodeNotFound(leaf_index),
        result.err().unwrap()
    );

    /**************************************
     * Fails if node is of node type
     **************************************/
    // Add node in tree
    let node_data = Nd { attr: 10 };

    let parent_index = 0;
    let _ = tree.add_node(parent_index, node_data.clone());

    // Try to remove
    let node_index = 2;
    let result = tree.remove_leaf(node_index);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::WrongNodeTypeError(node_index),
        result.err().unwrap()
    );

    /**************************************
     * Success
     **************************************/
    let leaf_index = 1;
    let result = tree.remove_leaf(leaf_index);

    assert_eq!(true, result.is_ok());

    // Should have 2 nodes at this point
    assert_eq!(2, tree.nodes().len());

    // Making sure that root children have been updated
    let result_root_children = tree.children(0);
    assert_eq!(true, result_root_children.is_some());
    let root_children: Vec<HoliumNode<Ld, Nd>> = result_root_children.unwrap();
    assert_eq!(1, root_children.len());

    let child = &root_children[0];
    // Root child should be index 1
    assert_eq!(1, child.index())
}

#[test]
fn test_remove_node() {
    /**************************************
     * Fails if tree root is node
     **************************************/
    let root_data = Nd { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((root_data, vec![]));

    let mut tree = HoliumTree::new(root_type).unwrap();

    let result = tree.remove_leaf(0);

    assert_eq!(true, result.is_err());
    assert_eq!(HoliumTreeError::RootNoRemovalError, result.err().unwrap());

    /**************************************
     * New tree and new node with leaf child
     **************************************/
    let root_data = Nd { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((root_data, vec![]));

    let mut tree = HoliumTree::new(root_type).unwrap();

    let node_data = Nd { attr: 10 };

    let parent_index = 0;
    let _ = tree.add_node(parent_index, node_data.clone());

    let leaf_data = Ld { attr: 10 };
    let _ = tree.add_leaf(1, leaf_data);

    /**************************************
     * Fails if node does not exists
     **************************************/
    let node_index = 10;
    let result = tree.remove_node(node_index);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::NodeNotFound(node_index),
        result.err().unwrap()
    );

    /**************************************
     * Fails if node is of leaf type
     **************************************/
    let node_index = 2;
    let result = tree.remove_node(node_index);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::WrongNodeTypeError(node_index),
        result.err().unwrap()
    );

    /**************************************
     * Success
     **************************************/
    let node_index = 1;
    let result = tree.remove_node(node_index);

    assert_eq!(true, result.is_ok());

    // Should have 1 node at this point, root as leaf is child of the node
    assert_eq!(1, tree.nodes().len());

    // Making sure that root children have been updated
    let result_root_children = tree.children(0);
    assert_eq!(true, result_root_children.is_none());
}

#[test]
fn test_update_leaf_data() {
    /**************************************
     * New tree and new leaf
     **************************************/
    let root_data = Nd { attr: 0 };

    let root_type: HoliumNodeType<Ld, Nd> = HoliumNodeType::Node((root_data, vec![]));

    let mut tree = HoliumTree::new(root_type).unwrap();

    let leaf_data = Ld { attr: 10 };

    let parent_index = 0;
    let _ = tree.add_leaf(parent_index, leaf_data.clone());

    /**************************************
     * Fails if node does not exists
     **************************************/
    let leaf_data = Ld { attr: 100 };

    let leaf_index = 10;
    let result = tree.update_leaf_data(leaf_index, leaf_data);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::NodeNotFound(leaf_index),
        result.err().unwrap()
    );

    /**************************************
     * Fails if node is of node type
     **************************************/
    let leaf_data = Ld { attr: 100 };

    let leaf_index = 0;
    let result = tree.update_leaf_data(leaf_index, leaf_data);

    assert_eq!(true, result.is_err());
    assert_eq!(
        HoliumTreeError::WrongNodeTypeError(leaf_index),
        result.err().unwrap()
    );

    /**************************************
     * Success
     **************************************/
    let leaf_data = Ld { attr: 100 };

    let leaf_index = 1;
    let result = tree.update_leaf_data(leaf_index, leaf_data.clone());

    assert_eq!(true, result.is_ok());

    let root_data: &Nd = match tree.node(0).unwrap().node_type() {
        HoliumNodeType::Node((data, _)) => data,
        _ => {
            panic!("root is supposed to be Node here")
        }
    };

    let node_type_should_be: HoliumNodeType<Ld, Nd> = HoliumNodeType::Leaf(leaf_data);

    // Making sure that root children have been updated
    let result_root_children = tree.children(0);
    assert_eq!(true, result_root_children.is_some());
    let root_children: Vec<HoliumNode<Ld, Nd>> = result_root_children.unwrap();
    assert_eq!(1, root_children.len());

    // Check that node is properly formed
    let root_child = &root_children[0];
    assert_eq!(&node_type_should_be, root_child.node_type());

    // Check that root has been updated par HoliumTreeData trait
    assert_eq!(100, root_data.attr);
}
