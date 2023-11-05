pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() < 1 {
        return vec![];
    }

    let mut result = vec![1; nums.len()];

    // Left
    let mut current = 1;
    for i in 0..nums.len() {
        result[i] = current;
        current *= nums[i];
    }

    // Result
    let mut current = nums[nums.len() - 1];

    for i in (0..nums.len() - 1).rev() {
        result[i] *= current;
        current *= nums[i];
    }

    result
}

#[cfg(test)]
mod product_except_self_test {
    use super::*;

    #[test]
    pub fn should_return_product_except_self() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(nums), vec![24, 12, 8, 6]);
    }

    #[test]
    pub fn should_return_empty() {
        let nums = vec![];
        assert_eq!(product_except_self(nums), vec![]);
    }

    #[test]
    pub fn should_return_one_element() {
        let nums = vec![1];
        assert_eq!(product_except_self(nums), vec![1]);
    }
}
