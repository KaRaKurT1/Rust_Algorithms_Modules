#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

pub fn in_order_traversal<T: std::fmt::Debug>(node: &Option<Box<TreeNode<T>>>) {
    if let Some(inner_node) = node {
        let node_ref = &*inner_node;
        in_order_traversal(&node_ref.left);
        println!("{:? }", node_ref.value);
        in_order_traversal(&node_ref.right);
    }
}