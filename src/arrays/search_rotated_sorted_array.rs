pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        // Are we in the left sorted portion
        if nums[mid] >= nums[left] {
            // Is the target in the left sorted portion
            if target > nums[mid] || target < nums[left] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
            // Are we in the right sorted portion
        } else {
            // Is the target in the right sorted portion
            if target > nums[mid] || target < nums[right] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod search_rotated_sorted_array_test {
    use super::*;

    #[test]
    pub fn should_return_search_rotated_sorted_array() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search(nums, 0), 4);
    }

    #[test]
    pub fn should_return_search_rotated_sorted_array_for_edge_case() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search(nums, 3), -1);
    }

    #[test]
    pub fn should_return_search_rotated_sorted_array_for_edge_case_2() {
        let nums = vec![1];
        assert_eq!(search(nums, 0), -1);
    }

    #[test]
    pub fn should_return_search_rotated_sorted_array_for_edge_case_3() {
        let nums = vec![1];
        assert_eq!(search(nums, 1), 0);
    }
}
