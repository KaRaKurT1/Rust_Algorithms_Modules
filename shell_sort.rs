pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    let mut h = 1;

    while h < len / 3 {
        h = 3 * h + 1; // Вычисление последовательности приращений (1, 4, 13, 40, ...)
    }

    while h >= 1 {
        for i in h..len {
            let mut j = i;
            while j >= h && arr[j] > arr[j - h] {
                arr.swap(j, j - h);
                j -= h;
            }
        }

        h /= 3; // Уменьшение приращения
    }
}