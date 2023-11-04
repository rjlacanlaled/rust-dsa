pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            if arr[j - 1] > arr[j] {
                swap(arr, j - 1, j);
            }
            j -= 1;
        }
    }
}

fn swap(arr: &mut [i32], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

#[cfg(test)]
mod insertion_sort_test {
    use super::*;

    #[test]
    pub fn should_sort() {
        let mut arr = [-1, 1, 2, 5, -10, 100, 5, 10];
        insertion_sort(&mut arr);
        assert_eq!(arr, [-10, -1, 1, 2, 5, 5, 10, 100]);
    }

    #[test]
    pub fn should_return_empty() {
        let mut arr = [];
        insertion_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    pub fn should_return_one_element() {
        let mut arr = [1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1]);
    }
}
