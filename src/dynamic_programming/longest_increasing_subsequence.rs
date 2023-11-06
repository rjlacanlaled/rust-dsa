use std::collections::BTreeSet;

// pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//     let mut dp: Vec<i32> = vec![1; nums.len()];
//     for i in (0..nums.len()).rev() {
//         for j in i..nums.len() {
//             if nums[i] < nums[j] {
//                 dp[i] = std::cmp::max(dp[i], dp[j] + 1);
//             }
//         }
//     }

//     return *dp.iter().max().unwrap();
// }

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut tree_set: BTreeSet<i32> = BTreeSet::new();

    for num in nums {
        let set = tree_set
            .range(num..)
            .next()
            .cloned();

        match set {
            Some(x) => {
                tree_set.remove(&x);
            }
            None => {
                tree_set.insert(num);
            }
        }

        tree_set.insert(num);
    }

    tree_set.len() as i32
}

#[cfg(test)]
mod longest_increasing_subsequence_test {
    use super::*;

    #[test]
    pub fn should_return_length_of_lis() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    pub fn should_return_length_of_lis_for_edge_case() {
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    pub fn should_return_length_of_lis_for_edge_case_2() {
        assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    pub fn should_return_length_of_lis_for_edge_case_3() {
        assert_eq!(length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    }
}
