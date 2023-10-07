// Определяем структуру для бинарного дерева поиска
#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value <= self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    fn inorder_traversal(&self, result: &mut Vec<T>) {
        if let Some(left) = &self.left {
            left.inorder_traversal(result);
        }
        result.push(self.value.clone());
        if let Some(right) = &self.right {
            right.inorder_traversal(result);
        }
    }
}

pub fn tree_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let mut root = TreeNode::new(arr[0].clone());

    for item in &arr[1..] {
        root.insert(item.clone());
    }

    let mut result = Vec::new();
    root.inorder_traversal(&mut result);
    result
}