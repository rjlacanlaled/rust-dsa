use core::borrow;
use std::{borrow::BorrowMut, ops::DerefMut};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(x), Some(y)) => {
                let sum = x.val + y.val;
                if sum > 9 {
                    let sum = sum - 10;
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(Solution::add_two_numbers(x.next, Some(Box::new(ListNode::new(1)))), y.next)
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(x.next, y.next)
                    }))
                }
            },
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (_, _) => None,
        }
    }
}


#[cfg(test)]
mod add_two_numbers_test {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4, 
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None
                }))
            }))
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6, 
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let expected = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0, 
                next: Some(Box::new(ListNode {
                    val: 8,
                    next: None
                }))
            }))
        }));

        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_add_two_numbers_2() {
        let l1 = Some(Box::new(ListNode {
            val: 0,
            next: None
        }));

        let l2 = Some(Box::new(ListNode {
            val: 0,
            next: None
        }));

        let expected = Some(Box::new(ListNode {
            val: 0,
            next: None
        }));

        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    //test for all 9
    #[test]
    fn test_add_two_numbers_3() {
        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9, 
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: None
                            }))
                        }))
                    }))
                }))
            }))
        }));

        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9, 
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: None
                    }))
                }))
            }))
        }));

        let expected = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9, 
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 1,
                                    next: None
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }));

        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

}