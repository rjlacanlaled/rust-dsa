struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = (nums.len() - 1) as i32;
        while low <= high {
            let mid = (low + high) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else {
                if nums[mid as usize] > target {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }   
        high + 1
    }
}

#[cfg(test)]
mod search_insert_position_test {
    use super::*;

    #[test]
    pub fn should_return_index() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 5), 2);
    }

    #[test]
    pub fn should_return_index_2() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 2), 1);
    }

    #[test]
    pub fn should_return_index_3() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 7), 4);
    }

    #[test]
    pub fn should_return_index_4() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 0), 0);
    }
}