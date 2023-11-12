use std::collections::{ HashMap, HashSet };

// array
mod arrays;
mod sorting;
mod binary;
mod dynamic_programming;

fn main() {
    // let mut root = TreeNode::new(1);
    // let mut root_left = TreeNode::new(2);
    // let mut root_right = TreeNode::new(2);
    // let root_left_left = TreeNode::new(3);
    // let root_left_right = TreeNode::new(4);
    // let root_right_left = TreeNode::new(4);
    // let root_right_right = TreeNode::new(3);

    // root_left.left = Some(Rc::new(RefCell::new(root_left_left)));
    // root_left.right = Some(Rc::new(RefCell::new(root_left_right)));
    // root_right.left = Some(Rc::new(RefCell::new(root_right_left)));
    // root_right.right = Some(Rc::new(RefCell::new(root_right_right)));
    // root.left = Some(Rc::new(RefCell::new(root_left)));
    // root.right = Some(Rc::new(RefCell::new(root_right)));

    // let root_node = Some(Rc::new(RefCell::new(root)));

    // let is_sym = Solutions::is_symmetric(root_node);

    // println!("{:?}", is_sym);
    // test_deque();
    let fib = fibonacci(99);
    println!("fib: {}", fib);
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

pub fn max_recursive(nums: Vec<i32>) -> i32 {
    if nums.len() < 1 {
        return 0;
    }

    if nums.len() == 1 {
        return nums[0];
    }

    nums[0].max(max_recursive(nums[1..].to_vec()))
}

pub fn binary_search_recurisve(nums: Vec<i32>, target: i32) -> bool {
    let low = 0;
    let high = (nums.len() - 1) as i32;

    if high >= low {
        let mid = (low + high) / 2;
        if nums[mid as usize] == target {
            return true;
        } else {
            if nums.len() < 2 {
                return false;
            }

            if nums[mid as usize] > target {
                return binary_search_recurisve(nums[..mid as usize].to_vec(), target);
            } else {
                return binary_search_recurisve(nums[(mid + 1) as usize..].to_vec(), target);
            }
        }
    }

    false
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(ref m), Some(ref n)) => {
            let m = m.borrow();
            let n = n.borrow();
            m.val == n.val &&
                is_same_tree(n.left.clone(), m.left.clone()) &&
                is_same_tree(n.right.clone(), m.right.clone())
        }
    }
}

pub fn quicksort_recursive(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() < 2 {
        return nums;
    }

    let pivot = nums[0];
    let left = nums[1..]
        .iter()
        .filter_map(|&x| if x <= pivot { Some(x) } else { None })
        .collect::<Vec<i32>>();
    let right = nums[1..]
        .iter()
        .filter_map(|&x| if x > pivot { Some(x) } else { None })
        .collect::<Vec<i32>>();

    // Recursively sort the left and right subarrays
    let left = quicksort_recursive(left);
    let right = quicksort_recursive(right);

    // Merge the sorted arrays
    [left, vec![pivot], right].concat()
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() < 1 {
        return None;
    }

    if nums.len() < 2 {
        return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
    }

    let mid = nums.len() / 2;
    let mut root = TreeNode::new(nums[mid]);
    root.left = sorted_array_to_bst(nums[..mid].to_vec());
    root.right = sorted_array_to_bst(nums[mid + 1..].to_vec());

    Some(Rc::new(RefCell::new(root)))
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut mapping: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let majority = (nums.len() as i32) / 2;

    for num in nums {
        *mapping.entry(num).or_insert(0) += 1;

        let count = mapping.get(&num).unwrap();
        if *count > majority {
            return num;
        }
    }

    0
}

struct Solutions;
impl Solutions {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                Self::is_symmetric_helper(node.borrow().left.clone(), node.borrow().right.clone())
            }
        }
    }

    fn is_symmetric_helper(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(m), Some(n)) => {
                let m = m.borrow();
                let n = n.borrow();
                m.val == n.val &&
                    Self::is_symmetric_helper(m.left.clone(), n.right.clone()) &&
                    Self::is_symmetric_helper(m.right.clone(), n.left.clone())
            }
            _ => false,
        }
    }
}

use std::collections::VecDeque;
pub fn test_deque() {
    let mut network = HashMap::new();
    network.insert("alice", vec!["bob", "claire"]);
    network.insert("bob", vec!["dylan", "claire"]);
    network.insert("claire", vec!["alice", "bob", "dylan"]);
    network.insert("dylan", vec!["alice"]);
    let mut queue = VecDeque::new();
    let mut searched = HashSet::new();

    queue.push_back("alice");

    println!("current queue: {:?}", queue);

    while queue.len() > 0 {
        let person = queue.pop_front();

        match person {
            None => println!("No more person"),
            Some(p) => {
                println!("searching {}", p);
                searched.insert(p);

                let person_network = network.get(p);

                match person_network {
                    None => println!("No network for {}", p),
                    Some(network) => {
                        for person in network {
                            if !searched.contains(person) {
                                queue.push_back(person);
                                continue;
                            }

                            println!("{} is already searched", person);
                        }
                    }
                }
            }
        }

        println!("new queue: {:?}", queue);
    }

    ()
}

pub fn fibonacci(n: u128) -> u128 {
    let mut dp: HashMap<u128, u128> = HashMap::new();

    fn fib(n: u128, dp: &mut HashMap<u128, u128>) -> u128 {
        // return cached data
        if let Some(memo) = dp.get(&n) {
            return *memo;
        }

        if n < 2 {
            return n;
        }

        println!("calculating fib({})", n);
        let result = fib(n - 1, dp) + fib(n - 2, dp);
        dp.insert(n, result);
        result
    }

    fib(n, &mut dp)

    // 1, 1, 2
}
