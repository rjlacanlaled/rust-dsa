pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut sorted_coins = coins.clone();
    sorted_coins.sort();

    let mut dp: Vec<i32> = vec![amount + 1; amount as usize + 1];
    dp[0] = 0;

    for i in 1..amount + 1 {
        for c in &coins {
            if i - c >= 0 {
                dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i - c) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] > amount {
        return -1;
    }

    dp[amount as usize]
}

#[cfg(test)]
mod coin_change_test {
    use super::*;

    #[test]
    pub fn should_return_coin_change() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    pub fn should_return_coin_change_for_edge_case() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }

    #[test]
    pub fn should_return_coin_change_for_edge_case_2() {
        assert_eq!(coin_change(vec![1], 0), 0);
    }

    #[test]
    pub fn should_return_coin_change_for_edge_case_3() {
        assert_eq!(coin_change(vec![1], 1), 1);
    }

    #[test]
    pub fn should_return_coin_change_for_edge_case_4() {
        assert_eq!(coin_change(vec![1], 2), 2);
    }
}
