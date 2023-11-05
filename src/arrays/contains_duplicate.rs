use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut num_mapping = HashSet::new();
    nums.iter().any(|x| !num_mapping.insert(x))
}

#[cfg(test)]
mod contains_duplicate_test {
    use super::*;

    #[test]
    pub fn should_return_true() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    pub fn should_return_false() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    pub fn should_return_true_for_empty() {
        let nums = vec![];
        assert_eq!(contains_duplicate(nums), false);
    }
}
