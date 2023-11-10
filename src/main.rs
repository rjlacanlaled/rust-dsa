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
    // let alpha = longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]);
    // println!("{}", alpha);
    dijkstra_shortest_path();
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
use std::thread::current;
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


pub fn largest_alphabetical_string(num: i32) -> String {
    const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
    let mut str_result: String = "".to_owned();
    let mut num = num;

    for i in (0..ALPHABET.len()).rev() {
        let letter_count = num / i32::pow(2, i as u32);
        num = num % i32::pow(2, i as u32);
        str_result.push_str(&(ALPHABET.as_bytes()[i as usize] as char).to_string().repeat(letter_count as usize));
    }

    str_result
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut lcp = strs[0].to_string();
    for str in strs[1..].to_vec() {
        let mut new_lcp = "".to_string();        
        if lcp.len() < 1 {
            return "".to_string();
        }
        for i in 0..lcp.len().min(str.len()) {
            if lcp.as_bytes()[i] == str.as_bytes()[i] {
                new_lcp.push_str(&(str.as_bytes()[i] as char).to_string());
                continue;
            }

            break;
        }
        lcp = new_lcp.clone();
    }
    lcp
}


pub fn dijkstra_shortest_path() -> HashMap<&'static str, (i32, Option<&'static str>)> {
    let mut nodes = HashMap::new();
    let mut path: HashMap<&str, (i32, Option<&str>)> = HashMap::new();
    let mut visited = HashSet::new();
    
    nodes.insert("a", [("b", 1), ("c", 3)]);
    nodes.insert("b", [("c", 1), ("d", 5)]);
    nodes.insert("c", [("e", 10), ("d", 2)]);

    path.insert("a", (0, None));
    path.insert("b", (i32::MAX, None));
    path.insert("c", (i32::MAX, None));
    path.insert("d", (i32::MAX, None));
    path.insert("e", (i32::MAX, None));

    let mut queue = VecDeque::new();
    queue.push_back("a");

    while queue.len() > 0 {
        println!("queue {:?}", queue);
        let current_node = queue.pop_front();

        println!("current node {:?}", current_node);
        match current_node {
            None => {},
            Some(node) => {
                if !visited.contains(node) {
                    visited.insert(node);
                    println!("visited nodes {:?}", visited);
                    println!("node {:?}", visited);
                    let neighbors = nodes.get(node);
                    if let Some(&x) = neighbors {
                        for i in 0..x.len() {
                            queue.push_back(x[i].0);
                            // Calculate the minimum path from node 1 -> node 2
                            let distance = match path.get(node) {
                                None => (0, None),
                                Some(p) => {
                                    let current_path = path.get(x[i].0);
                                    println!("current_path {:?}", current_path);
                                    // Shortest distance via parent node
                                    ((p.0 + x[i].1).min(path.get(x[i].0).unwrap().0), Some(node))
                                }
                            };
                            path.insert(x[i].0, distance);
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", path);

    path

}