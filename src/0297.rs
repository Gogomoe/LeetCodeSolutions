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
use std::ops::Add;

struct Codec {}


/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut list = Vec::new();
        let mut level = vec![root.clone()];
        while !level.is_empty() {
            let mut next = Vec::new();
            for node in level {
                match node {
                    Some(it) => {
                        list.push(it.borrow_mut().val.to_string());
                        next.push(it.borrow_mut().left.clone());
                        next.push(it.borrow_mut().right.clone());
                    }
                    None => {
                        list.push("null".to_string());
                    }
                }
            }
            level = next;
        }
        return "[".to_string().add(&list.join(",")).add("]");
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let strs: Vec<&str> = data[1..data.len() - 1].split(",")
            .filter(|it| !it.is_empty())
            .collect();
        if strs.is_empty() || (strs.len() == 1 && strs[0] == "null") {
            return None;
        }
        let mut root = Rc::new(RefCell::new(TreeNode::new(strs[0].parse().unwrap())));
        let mut i = 1;
        let mut level = vec![root.clone()];
        while i != strs.len() {
            let mut next = Vec::new();
            for node in level {
                if i == strs.len() { break; }
                node.borrow_mut().left = if strs[i] == "null" {
                    None
                } else {
                    let sub_node = Rc::new(RefCell::new(TreeNode::new(strs[i].parse().unwrap())));
                    next.push(sub_node.clone());
                    Some(sub_node)
                };
                i += 1;
                if i == strs.len() { break; }
                node.borrow_mut().right = if strs[i] == "null" {
                    None
                } else {
                    let sub_node = Rc::new(RefCell::new(TreeNode::new(strs[i].parse().unwrap())));
                    next.push(sub_node.clone());
                    Some(sub_node)
                };
                i += 1;
            }
            level = next;
        }

        return Some(root);
    }
}


struct Solution {}

fn main() {
    println!("{:?}", Codec::new().deserialize("[]".to_string())); // None
}

