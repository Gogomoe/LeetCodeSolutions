// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
use std::cmp::max;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return max_path_sum_with_root(&root).0;
    }
}

fn max_path_sum_with_root(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() { return (i32::MIN / 10, i32::MIN / 10); }
    let root = root.as_ref().unwrap();

    let left = max_path_sum_with_root(&root.borrow_mut().left);
    let right = max_path_sum_with_root(&root.borrow_mut().right);

    let single_root_result = root.borrow_mut().val + max(max(left.1, 0), max(right.1, 0));
    let both_root_result = max(left.1, 0) + max(right.1, 0) + root.borrow_mut().val;
    let max_result = max(max(left.0, right.0), both_root_result);

    return (max_result, single_root_result);
}

struct Solution {}

fn main() {
    println!("{}", Solution::max_path_sum(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    }))))); // 6

    println!("{}", Solution::max_path_sum(Some(Rc::new(RefCell::new(TreeNode {
        val: -10,
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
    }))))); // 42

    println!("{}", Solution::max_path_sum(Some(Rc::new(RefCell::new(TreeNode {
        val: -3,
        left: None,
        right: None,
    })))));
}