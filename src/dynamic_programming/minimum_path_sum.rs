use std::f32::INFINITY;

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut dp: Vec<Vec<i32>> = grid
        .into_iter()
        .map(|row| {
            let mut row = row;
            row.push(f32::INFINITY as i32);
            row
        })
        .collect();
    dp.push(vec![f32::INFINITY as i32; cols + 1]);
    dp[rows][cols - 1] = 0;

    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            dp[i][j] = dp[i + 1][j].min(dp[i][j + 1]) + dp[i][j];
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod min_path_sum_test {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]), 7);
    }

    #[test]
    fn test_min_path_sum_2() {
        assert_eq!(min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }

    #[test]
    fn test_min_path_sum_3() {
        assert_eq!(min_path_sum(vec![vec![1, 2], vec![1, 1]]), 3);
    }

    #[test]
    fn test_min_path_sum_4() {
        assert_eq!(min_path_sum(vec![vec![1, 2, 5], vec![3, 2, 1]]), 6);
    }
}
