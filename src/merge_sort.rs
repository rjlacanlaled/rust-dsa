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

    // Compare the left and right parts
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

    // Copy the remaining left elements
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    // Copy the remaining right elements
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod merge_sort_test {
    pub use super::*;

    #[test]
    pub fn should_sort() {
        let mut arr = [-1, 1, 2, 5, -10, 100, 5, 10];
        merge_sort(&mut arr);
        assert_eq!(arr, [-10, -1, 1, 2, 5, 5, 10, 100]);
    }

    #[test]
    pub fn should_return_empty() {
        let mut arr = [];
        merge_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    pub fn should_return_one_element() {
        let mut arr = [1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1]);
    }
}
