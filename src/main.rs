use std::collections::HashMap;

// array
mod arrays;
mod sorting;
mod binary;
mod dynamic_programming;

fn main() {
    let nums = [];

    // let max_average = find_max_average(vec![1, 12, -5, -6, 50, 3], 4);
    // println!("{}", max_average);
    let sum = add_recursive(nums.to_vec());
    println!("{}", sum);
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut visited: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let duplicate = visited.get(&nums[i]);
        match duplicate {
            Some(v) => {
                if (v - (i as i32)).abs() <= k {
                    return true;
                } else {
                    visited.insert(nums[i], i as i32);
                }
            }
            None => {
                visited.insert(nums[i], i as i32);
            }
        }
    }

    false
}

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut left = 0;
    let mut right = k as usize;
    let mut current_sum = 0.0;

    for i in left..=right - 1 {
        current_sum += nums[i] as f64;
    }

    let mut max_average = current_sum / (k as f64);

    while right < nums.len() {
        current_sum -= nums[left] as f64;
        current_sum += nums[right] as f64;
        max_average = max_average.max(current_sum / (k as f64));
        left += 1;
        right += 1;
    }

    max_average
}

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut basket: HashMap<i32, i32> = HashMap::new();

    let mut left = 0;
    let mut right = 1;
    let mut max_fruits = 0;

    // 1, 2, 1
    while left < right {
        // Add the first fruit to the basket
        // [1] = 1

    }

    0
}

pub fn add_recursive(nums: Vec<i32>) -> i32 {
    if nums.len() < 1 {
        return 0;
    }

    if nums.len() == 1 {
        return nums[0];
    }

    nums[0] + add_recursive(nums[1..].to_vec())
}
