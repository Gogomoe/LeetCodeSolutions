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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        return generate(1, n);
    }
}

fn generate(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if start > end {
        return vec![None];
    }
    let mut result = Vec::new();
    for i in start..=end {
        let left = generate(start, i - 1);
        let right = generate(i + 1, end);

        for l in left.iter() {
            for r in right.iter() {
                let mut node = TreeNode::new(i);
                node.left = l.clone();
                node.right = r.clone();
                result.push(Some(Rc::new(RefCell::new(node))));
            }
        }
    }
    return result;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::generate_trees(3));
    println!("{:?}", Solution::generate_trees(1));
}
