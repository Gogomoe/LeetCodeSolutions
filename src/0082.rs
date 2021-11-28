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

use leetcode_solution::{ListNode, make_list, print_list};

fn main() {
    print_list(Solution::delete_duplicates(make_list(vec![1, 2, 3, 3, 4, 4, 5]))); // [1,2,5]
    print_list(Solution::delete_duplicates(make_list(vec![1, 1, 1, 2, 3]))); // [2,3]
    print_list(Solution::delete_duplicates(make_list(vec![1, 1]))); // []
}
