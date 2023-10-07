pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return; // Базовый случай: если массив пустой или содержит только один элемент, он уже отсортирован
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);

    quick_sort(left);
    quick_sort(&mut right[1..]); // Исключаем pivot, так как он уже на своей позиции
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2; // Выбираем средний элемент в качестве опорного
    arr.swap(pivot_index, len - 1); // Помещаем опорный элемент в конец массива
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1); // Помещаем опорный элемент на его окончательную позицию
    i
}