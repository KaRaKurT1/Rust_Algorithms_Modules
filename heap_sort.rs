pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    // Строим максимальную пирамиду
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    // Извлекаем элементы из пирамиды и сортируем
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], len: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < len && arr[left] > arr[largest] {
        largest = left;
    }

    if right < len && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != root {
        arr.swap(root, largest);
        heapify(arr, len, largest);
    }
}