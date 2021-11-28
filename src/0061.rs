impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            count += 1;
            curr = node.next.as_ref();
        }

        if count == 0 {
            return head;
        }
        k %= count;
        if k == 0 {
            return head;
        }

        let mut curr = &mut head;
        for _ in 0..(count - k) {
            curr = &mut curr.as_mut().unwrap().next;
        }

        let mut tail = curr.take();
        let mut curr = &mut tail;
        for _ in 0..k {
            curr = &mut curr.as_mut().unwrap().next;
        }
        *curr = head;
        return tail;
    }
}

struct Solution {}

use leetcode_solution::{ListNode, make_list, print_list};

fn main() {
    print_list(Solution::rotate_right(make_list(vec![1, 2, 3, 4, 5]), 2)); // [4,5,1,2,3]
    print_list(Solution::rotate_right(make_list(vec![0, 1, 2]), 4)); // [2,0,1]
}