use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

impl Ord for Box<ListNode> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for Box<ListNode> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(-1);
        let mut last = &mut head.next;

        let mut heap = BinaryHeap::new();
        for list in lists {
            if let Some(node) = list {
                heap.push(node)
            }
        }

        while !heap.is_empty() {
            let mut list = heap.pop().unwrap();
            let mut next = list.next.take();

            *last = Some(list);
            last = &mut (*last).as_mut().unwrap().next;

            if let Some(node) = next {
                heap.push(node)
            }
        }

        head.next
    }
}

struct Solution {}

fn main() {}