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

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return is_valid_bst_ref(root.as_ref().unwrap(), i32::MIN, i32::MAX);
    }
}

fn is_valid_bst_ref(root: &Rc<RefCell<TreeNode>>, bottom: i32, up: i32) -> bool {
    let node = root;
    let val = node.borrow_mut().val;
    if val > up || val < bottom {
        return false;
    }
    let mut result = true;
    if let Some(left) = (&node.borrow_mut().left).as_ref() {
        if left.borrow_mut().val >= val {
            return false;
        }
        result &= is_valid_bst_ref(left, bottom, val - 1);
    }
    if let Some(right) = (&node.borrow_mut().right).as_ref() {
        if right.borrow_mut().val <= val {
            return false;
        }
        result &= is_valid_bst_ref(right, val + 1, up);
    }
    return result;
}

struct Solution {}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    println!("{}", Solution::is_valid_bst(tree));
}
