use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let symbol_to_value = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        
        let mut result = 0;

        s.chars().rev().fold(0, |acc, c| {
            let value = symbol_to_value.get(&c).unwrap();
            if *value < acc {
                result -= value;
            } else {
                result += value;
            }
            *value
        });

        result
    }
}


#[cfg(test)]
mod roman_to_int_test {
    use super::*;

    #[test]
    pub fn should_return_3() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    pub fn should_return_4() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }

    #[test]
    pub fn should_return_9() {
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    }

    #[test]
    pub fn should_return_58() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    pub fn should_return_1994() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}