// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = &mut head;
        let mut is_dup = false;
        while node.is_some() && node.as_ref().unwrap().next.is_some() {
            if node.as_ref().unwrap().val == node.as_ref().unwrap().next.as_ref().unwrap().val {
                is_dup = true;
                node.as_mut().unwrap().next = node.as_mut().unwrap().next.as_mut().unwrap().next.take();
            } else {
                if is_dup {
                    *node = node.as_mut().unwrap().next.take();
                } else {
                    node = &mut node.as_mut().unwrap().next;
                }
                is_dup = false;
            }
        }
        if is_dup {
            *node = None;
        }
        return head;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::delete_duplicates(make_list(vec![1, 2, 3, 3, 4, 4, 5]))); // [1,2,5]
    println!("{:?}", Solution::delete_duplicates(make_list(vec![1, 1, 1, 2, 3]))); // [2,3]
    println!("{:?}", Solution::delete_duplicates(make_list(vec![1, 1]))); // []
}

fn make_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut list = None;
    for value in vec.into_iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = list;
        list = Some(node);
    }
    return list;
}