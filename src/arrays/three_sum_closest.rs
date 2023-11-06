pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut result = sorted_nums[0] + sorted_nums[1] + sorted_nums[nums.len() - 1];

    for i in 0..sorted_nums.len() - 1 {
        let a = sorted_nums[i];

        let mut left = i + 1;
        let mut right = sorted_nums.len() - 1;

        while left < right {
            let sum = a + sorted_nums[left] + sorted_nums[right];
            let current_result_diff = (target - result).abs();
            let sum_diff = (target - sum).abs();

            if sum_diff < current_result_diff {
                result = sum;
            }

            if sum == target {
                return sum;
            }

            if target - sum > 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    result
}

use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let hash_set: HashSet<i32> = HashSet::from_iter(nums.clone());
    hash_set.len() as i32
}

#[cfg(test)]
mod three_sum_closest_test {
    use super::*;

    #[test]
    pub fn should_return_three_sum_closest() {
        let nums = vec![-1, 2, 1, -4];
        assert_eq!(three_sum_closest(nums, 1), 2);
    }

    #[test]
    pub fn should_return_three_sum_closest_for_edge_case() {
        let nums = vec![1, 1, 1, 0];
        assert_eq!(three_sum_closest(nums, -100), 2);
    }

    #[test]
    pub fn should_return_three_sum_closest_for_edge_case_2() {
        let nums = vec![0, 2, 1, -3];
        assert_eq!(three_sum_closest(nums, 1), 0);
    }

    #[test]
    pub fn should_return_three_sum_closest_for_edge_case_3() {
        let nums = vec![1, 1, -1, -1, 3];
        assert_eq!(three_sum_closest(nums, -1), -1);
    }

    #[test]
    pub fn should_return_three_sum_closest_for_edge_case_4() {
        let nums = vec![1, 1, -1, -1, 3];
        assert_eq!(three_sum_closest(nums, 3), 3);
    }
}
