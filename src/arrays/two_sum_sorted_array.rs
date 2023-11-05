pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted_num = numbers.clone();
    sorted_num.sort();

    let mut left = 0;
    let mut right = sorted_num.len() - 1;

    while left < right {
        let sum = sorted_num[left] + sorted_num[right];

        if sum == target {
            return vec![(left as i32) + 1, (right as i32) + 1];
        }

        if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    vec![]
}

#[cfg(test)]
mod two_sum_sorted_array_test {
    use super::*;

    #[test]
    pub fn should_return_two_sum_sorted_array() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums, 9), vec![1, 2]);
    }

    #[test]
    pub fn should_return_two_sum_sorted_array_for_edge_case() {
        let nums = vec![2, 3, 4];
        assert_eq!(two_sum(nums, 6), vec![1, 3]);
    }

    #[test]
    pub fn should_return_two_sum_sorted_array_for_edge_case_2() {
        let nums = vec![-1, 0];
        assert_eq!(two_sum(nums, -1), vec![1, 2]);
    }

    #[test]
    pub fn should_return_two_sum_sorted_array_for_edge_case_3() {
        let nums = vec![0, 0, 3, 4];
        assert_eq!(two_sum(nums, 0), vec![1, 2]);
    }
}
