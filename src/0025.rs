impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        for _ in 0..k {
            match tail.as_mut() {
                Some(node) => { tail = &mut node.next; }
                None => { return head; }
            }
        }
        let tail = tail.take();

        let mut curr = head.unwrap();
        let mut next = curr.next.take();
        curr.next = Solution::reverse_k_group(tail, k);
        while let Some(mut node) = next {
            let mut temp = node.next.take();
            node.next = Some(curr);
            curr = node;
            next = temp;
        }

        return Some(curr);
    }
}

struct Solution {}

use leetcode_solution::{ListNode};

fn main() {}