pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    if sorted_nums[0] > 0 {
        return vec![];
    }

    let mut result = vec![];

    for i in 0..sorted_nums.len() - 1 {
        let a = sorted_nums[i];

        if i > 0 && sorted_nums[i] == sorted_nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = sorted_nums.len() - 1;

        while left < right {
            let sum = a + sorted_nums[left] + sorted_nums[right];

            if sum == 0 {
                result.push(vec![a, sorted_nums[left], sorted_nums[right]]);

                // skip duplicates
                left += 1;
                while sorted_nums[left] == sorted_nums[left - 1] && left < right {
                    left += 1;
                }
            } else if sum > 0 {
                right -= 1;
            } else {
                left += 1;
            }
        }
    }

    result
}
