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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let mut tail = &mut result;
        let mut curr = &mut head;
        while curr.is_some() {
            if curr.as_ref().unwrap().val < x {
                let next = curr.as_mut().unwrap().next.take();
                *tail = curr.take();
                *curr = next;
                tail = &mut tail.as_mut().unwrap().next;
            } else {
                curr = &mut curr.as_mut().unwrap().next;
            }
        }
        *tail = head;

        return result;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::partition(make_list(vec![1, 4, 3, 2, 5, 2]), 3)); // [1,2,2,4,3,5]
    println!("{:?}", Solution::partition(make_list(vec![2, 1]), 2)); // [1,2]
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