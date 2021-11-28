impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut start = ListNode::new(-1);
        start.next = head;

        let mut fast = &start.next;
        for _ in 0..n {
            match fast.as_ref() {
                Some(node) => { fast = &node.next }
                None => { return start.next; }
            }
        }
        let mut slow = &start.next as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;

        while let Some(node) = fast.as_ref() {
            fast = &node.next;
            slow = &mut unsafe { (*slow).as_mut() }.unwrap().next as *mut Option<Box<ListNode>>;
        }

        unsafe {
            *slow = (*slow).as_mut().unwrap().next.take();
        }

        return start.next;
    }
}

struct Solution {}

use leetcode_solution::{ListNode, make_list, print_list};

fn main() {
    print_list(Solution::remove_nth_from_end(make_list(vec![1, 2, 3, 4, 5]), 2)); // [1,2,3,5]
    print_list(Solution::remove_nth_from_end(make_list(vec![1]), 1)); // []
}