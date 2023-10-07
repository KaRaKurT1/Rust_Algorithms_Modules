// Определяем структуру для бинарного дерева
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

// Функция для обхода дерева в порядке "pre-order"
pub fn pre_order_traversal<T>(node: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>)
where
    T: Clone,
{
    if let Some(inner_node) = node {
        result.push(inner_node.value.clone()); // Посещаем корень
        pre_order_traversal(&inner_node.left, result); // Рекурсивно обходим левое поддерево
        pre_order_traversal(&inner_node.right, result); // Рекурсивно обходим правое поддерево
    }
}