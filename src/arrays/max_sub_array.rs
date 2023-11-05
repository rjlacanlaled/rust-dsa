pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() < 1 {
        return 0;
    }

    let mut current_sum = nums[0];
    let mut best_sum = nums[0];

    for i in 1..nums.len() {
        if current_sum < 0 {
            current_sum = 0;
        }

        current_sum += nums[i];
        best_sum = std::cmp::max(best_sum, current_sum);
    }

    best_sum
}

#[cfg(test)]
mod max_sub_array_test {
    use super::*;

    #[test]
    pub fn should_return_max_sub_array() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(nums), 6);
    }

    #[test]
    pub fn should_return_max_sub_array_for_edge_case() {
        let nums = vec![-2, 1];
        assert_eq!(max_sub_array(nums), 1);
    }

    #[test]
    pub fn should_return_empty() {
        let nums = vec![];
        assert_eq!(max_sub_array(nums), 0);
    }

    #[test]
    pub fn should_return_one_element() {
        let nums = vec![1];
        assert_eq!(max_sub_array(nums), 1);
    }
}
