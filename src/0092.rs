impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }
        let mut node = &mut head;
        for _ in 0..left - 1 {
            node = &mut node.as_mut().unwrap().next;
        }

        let list = node.take();
        *node = reverse_first_k_nodes(list, right - left);

        return head;
    }
}

fn reverse_first_k_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut p1 = head;
    let mut p2 = p1.as_mut().unwrap().next.take();
    let head = p1.as_mut().unwrap().as_mut() as *mut ListNode;
    for _ in 0..k {
        let mut p3 = p2.as_mut().unwrap().next.take();
        p2.as_mut().unwrap().next = p1;
        p1 = p2;
        p2 = p3;
    }
    unsafe {
        (*head).next = p2.take();
    }
    return p1;
}

struct Solution {}

use leetcode_solution::{ListNode, make_list, print_list};

fn main() {
    print_list(Solution::reverse_between(make_list(vec![1, 2, 3, 4, 5]), 2, 4));
    // [1,4,3,2,5]
    print_list(Solution::reverse_between(make_list(vec![1, 2, 3, 4, 5]), 2, 5));
    // [1,5,4,3,2]
    print_list(Solution::reverse_between(make_list(vec![1, 2, 3, 4, 5]), 1, 4));
    // [4,3,2,1,5]
}