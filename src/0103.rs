// Definition for a binary tree node.
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


use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() { return Vec::new(); }
        let mut result = Vec::new();
        let mut left_to_right = true;
        let mut queue = vec![root.as_ref().unwrap().clone()];
        let mut next = VecDeque::new();
        while !queue.is_empty() {
            let mut numbers = Vec::new();
            for it in queue {
                numbers.push(it.borrow_mut().val);
                if left_to_right {
                    if it.borrow_mut().left.is_some() {
                        next.push_front(it.borrow_mut().left.as_ref().unwrap().clone());
                    }
                    if it.borrow_mut().right.is_some() {
                        next.push_front(it.borrow_mut().right.as_ref().unwrap().clone());
                    }
                } else {
                    if it.borrow_mut().right.is_some() {
                        next.push_front(it.borrow_mut().right.as_ref().unwrap().clone());
                    }
                    if it.borrow_mut().left.is_some() {
                        next.push_front(it.borrow_mut().left.as_ref().unwrap().clone());
                    }
                }
            }

            result.push(numbers);
            left_to_right = !left_to_right;
            queue = next.into_iter().collect();
            next = VecDeque::new();
        }

        return result;
    }
}

struct Solution {}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    println!("{:?}", Solution::zigzag_level_order(tree)); // [[3],[20,9],[15,7]]

    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    println!("{:?}", Solution::zigzag_level_order(tree)); // [[1]]

    let tree = None;
    println!("{:?}", Solution::zigzag_level_order(tree)); // []
}
