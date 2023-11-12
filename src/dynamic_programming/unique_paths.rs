pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize + 1]; m as usize + 1];
    dp[(m as usize) - 1][(n as usize) - 1] = 1;

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            let row = i as usize;
            let col = j as usize;
            dp[row][col] = dp[row + 1][col] + dp[row][col + 1] + dp[row][col];
        }
    }
    dp[0][0]
}

#[cfg(test)]
mod unique_paths_test {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn test_unique_paths_2() {
        assert_eq!(unique_paths(7, 3), 28);
    }

    #[test]
    fn test_unique_paths_3() {
        assert_eq!(unique_paths(3, 3), 6);
    }

    #[test]
    fn test_unique_paths_4() {
        assert_eq!(unique_paths(1, 1), 1);
    }
}
