use std::collections::HashMap;

pub fn index_sequential_search<T: Eq + std::hash::Hash + Clone>(
    data: &[T],
    index: &HashMap<T, usize>,
    target: &T,
) -> Option<usize> {
    match index.get(target) {
        Some(&index) => {
            if index < data.len() && data[index] == *target {
                Some(index)
            } else {
                None
            }
        }
        None => None,
    }
}