pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![1; nums.len()];
    for i in (0..nums.len()).rev() {
        for j in i..nums.len() {
            if nums[i] < nums[j] {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }
    }

    return *dp.iter().max().unwrap();
}
