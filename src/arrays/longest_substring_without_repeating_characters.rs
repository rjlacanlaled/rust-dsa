use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut curr = 0;
        let mut set = HashSet::new();
        let mut l = 0;
        let mut r = 0;

        while r < s.len() {
            let res = set.insert(s.as_bytes()[r]);

            if res {
                curr += 1;
                max = max.max(curr);
            } else {
                while s.as_bytes()[l] != s.as_bytes()[r] {
                    set.remove(&s.as_bytes()[l]);
                    l += 1;
                    curr -= 1;
                }

                l += 1;
            }

            r += 1;
        }

    
        max.max(curr)
    }
}

#[cfg(test)]
mod longest_substring_without_repeating_characters_test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    }
}