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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = VecDeque::new();
        dfs(root, &mut result, 0);
        return result.into_iter().collect();
    }
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, result: &mut VecDeque<Vec<i32>>, level: i32) {
    if node.is_none() { return; }
    let node = node.unwrap();
    if result.len() == level as usize {
        result.push_front(Vec::new());
    }
    dfs(node.borrow_mut().left.take(), result, level + 1);
    dfs(node.borrow_mut().right.take(), result, level + 1);
    let len = result.len();
    result.get_mut(len - 1 - level as usize).unwrap().push(node.borrow_mut().val);
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
    println!("{:?}", Solution::level_order_bottom(tree)); // [[15,7],[9,20],[3]]

    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    println!("{:?}", Solution::level_order_bottom(tree)); // [[1]]

    let tree = None;
    println!("{:?}", Solution::level_order_bottom(tree)); // []
}
