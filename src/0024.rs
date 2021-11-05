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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut head = head.unwrap();
        let mut next = head.next.take().unwrap();
        let tail = next.next.take();

        head.next = Solution::swap_pairs(tail);
        next.next = Some(head);

        return Some(next);
    }
}

struct Solution {}

fn main() {
    let node5 = ListNode::new(5);
    let mut node4 = ListNode::new(4);
    let mut node3 = ListNode::new(3);
    let mut node2 = ListNode::new(2);
    let mut node1 = ListNode::new(1);

    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));

    println!("{:?}", Solution::swap_pairs(Some(Box::new(node1))));
}