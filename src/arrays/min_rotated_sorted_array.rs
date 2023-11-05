pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut mid = nums.len() / 2;
    let mut right = nums.len() - 1;
    let mut left = 0;

    let mut min = nums[mid];

    loop {
        if nums[right] >= nums[left] {
            return std::cmp::min(min, nums[left]);
        }

        if nums[mid] >= nums[left] {
            // search right
            left = mid + 1;
            mid = (left + right) / 2;
            min = std::cmp::min(min, nums[mid]);
        } else {
            // search left
            right = mid - 1;
            mid = (left + right) / 2;
            min = std::cmp::min(min, nums[mid]);
        }
    }
}

#[cfg(test)]
mod min_rotated_sorted_array_test {
    use super::*;

    #[test]
    pub fn should_return_min_rotated_sorted_array() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(find_min(nums), 0);
    }

    #[test]
    pub fn should_return_min_rotated_sorted_array_for_edge_case() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    pub fn should_return_min_rotated_sorted_array_for_edge_case_2() {
        let nums = vec![1, 2];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    pub fn should_return_min_rotated_sorted_array_for_edge_case_3() {
        let nums = vec![2, 1];
        assert_eq!(find_min(nums), 1);
    }
}
