use holium_utils::tree::HoliumNodeType::Leaf;
use holium_utils::tree::{HoliumNode, HoliumNodeType, HoliumTree, HoliumTreeData};

#[test]
fn test_trait() {
    struct LeafData {
        a: u32,
    }

    struct NodeData {
        b: u32,
    }

    impl HoliumTreeData<LeafData, NodeData> for NodeData {
        fn on_new_child(&mut self, child: &HoliumNode<LeafData, NodeData>) -> &mut Self {
            print!("ON NEW CHILD");

            self
        }

        fn on_child_update(&mut self, child: &HoliumNode<LeafData, NodeData>) -> &mut Self {
            print!("ON CHILD UPDATE");

            self
        }

        fn on_child_removed(&mut self, child: &HoliumNode<LeafData, NodeData>) -> &mut Self {
            print!("ON CHILD REMOVED");

            self
        }
    }

    let root_data = NodeData { b: 0 };
    let root_type = HoliumNodeType::Node((NodeData, vec![]));
    let mut tree: HoliumTree<LeafData, NodeData> = HoliumTree::new(root_type);
    tree.add_leaf(0, LeafData { a: 15 });
}
