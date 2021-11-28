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

use leetcode_solution::{ListNode, make_list, print_list};

fn main() {
    print_list(Solution::partition(make_list(vec![1, 4, 3, 2, 5, 2]), 3)); // [1,2,2,4,3,5]
    print_list(Solution::partition(make_list(vec![2, 1]), 2)); // [1,2]
}
