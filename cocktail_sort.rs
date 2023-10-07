pub fn cocktail_sort<T: Ord>(arr: &mut [T]) {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut swapped = true;

    while left < right && swapped {
        swapped = false;

        for i in left..right {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        right -= 1;

        for i in (left..right).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        left += 1;
    }
}