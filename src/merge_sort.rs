pub fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len > 1 {
        // Split in half
        let mid = len / 2;

        // Get the left part
        let left = &mut arr[..mid].to_vec();

        // Get the right part
        let right = &mut arr[mid..].to_vec();

        // Sort the left part recursively
        merge_sort(left);

        // Sort the right part recursively
        merge_sort(right);

        // Merge the result of both
        merge(left, right, arr);
    }
}

fn merge(left: &mut [i32], right: &mut [i32], arr: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }

        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
