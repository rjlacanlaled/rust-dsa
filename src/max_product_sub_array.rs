pub fn max_product(nums: Vec<i32>) -> i32 {
    // using kadane's algorithm
    let mut best_product = std::cmp::max(nums[0], nums[nums.len() - 1]);
    let mut current_prefix_product = nums[0];
    let mut current_suffix_product = nums[nums.len() - 1];

    for i in 1..nums.len() {
        if current_prefix_product == 0 {
            current_prefix_product = 1;
        }

        if current_suffix_product == 0 {
            current_suffix_product = 1;
        }

        current_prefix_product *= nums[i];
        current_suffix_product *= nums[nums.len() - 1 - i];

        best_product = std::cmp::max(
            best_product,
            std::cmp::max(current_prefix_product, current_suffix_product)
        );
    }

    best_product
}
