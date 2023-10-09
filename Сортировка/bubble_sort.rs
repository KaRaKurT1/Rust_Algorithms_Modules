pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut swapped;
    
    for i in 0..n {
        swapped = false;
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
                swapped = true;
            }
        }
        
        // Если во внутреннем цикле не было обменов,
        // значит массив уже отсортирован
        if !swapped {
            break;
        }
    }
}
