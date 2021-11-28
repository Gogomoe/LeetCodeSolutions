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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = head.as_ref();
        let mut len = 0;
        while node.is_some() {
            node = node.unwrap().next.as_ref();
            len += 1;
        }

        return build_bst(head, len).0;
    }
}

fn build_bst(head: Option<Box<ListNode>>, len: i32) -> (Option<Rc<RefCell<TreeNode>>>, Option<Box<ListNode>>) {
    if len == 0 {
        return (None, head);
    }

    let (left, mut head) = build_bst(head, (len - 1) / 2);
    let mut node = TreeNode::new(head.as_ref().unwrap().val);
    head = head.as_mut().unwrap().next.take();
    let (right, head) = build_bst(head, len - 1 - (len - 1) / 2);
    node.left = left;
    node.right = right;
    return (Some(Rc::new(RefCell::new(node))), head);
}

struct Solution {}

use leetcode_solution::{ListNode, make_list};

fn main() {
    println!("{:?}", Solution::sorted_list_to_bst(make_list(vec![-10, -3, 0, 5, 9])));
    println!("{:?}", Solution::sorted_list_to_bst(make_list(vec![]))); // []
    println!("{:?}", Solution::sorted_list_to_bst(make_list(vec![0]))); // [0]
    println!("{:?}", Solution::sorted_list_to_bst(make_list(vec![1, 3])));
}
