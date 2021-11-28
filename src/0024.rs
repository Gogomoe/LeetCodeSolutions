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

use leetcode_solution::{ListNode, make_list, print_list};

fn main() {
    print_list(Solution::swap_pairs(make_list(vec![1, 2, 3, 4]))); // [2,1,4,3]
}