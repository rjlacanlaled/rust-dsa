// array
mod arrays;
mod sorting;
mod binary;
mod dynamic_programming;

fn main() {
    let mut arr = [0, 0, 0, 1, 10, 100, -2, -10, 5, 5, 10, 100];

    // insertion_sort::insertion_sort(&mut arr);
    // println!("insertion_sort {:?}", arr);

    // merge_sort::merge_sort(&mut arr);
    // println!("merge_sort {:?}", arr);

    // quick_sort::quick_sort(&mut arr);
    // println!("quick_sort {:?}", arr)

    // let min = arrays::min_rotated_sorted_array::find_min(arr.to_vec());
    // println!("min {:?}", min);

    // let target = arrays::three_sum::three_sum(arr.to_vec());
    // println!("target {:?}", target);

    // let sum = arrays::container_with_most_water::max_area(arr.to_vec());
    // println!("sum {:?}", sum);

    // let sum = arrays::three_sum_closest::three_sum_closest(arr.to_vec(), 199);
    // println!("sum {:?}", sum)

    let steps = dynamic_programming::longest_increasing_subsequence::length_of_lis(
        vec![10, 9, 2, 5, 3, 7, 101, 18]
    );
    println!("steps {:?}", steps);
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
      right: None
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
        (Some(ref m), Some(ref n))=> {
            let m = m.borrow();
            let n = n.borrow();
            m.val == n.val && is_same_tree(n.left.clone(), m.left.clone()) && is_same_tree(n.right.clone(), m.right.clone())
        }
    }
}