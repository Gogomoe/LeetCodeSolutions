use std::ptr::null_mut;

impl Solution {
    fn make_ptr(l: &Option<Box<ListNode>>) -> *mut ListNode {
        match l {
            Some(x) => { &**x as *const ListNode as *mut ListNode }
            None => null_mut()
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unsafe {
            let mut head = ListNode::new(0);
            let mut last = &mut head as *mut ListNode;
            let mut p1 = Solution::make_ptr(&l1);
            let mut p2 = Solution::make_ptr(&l2);
            let mut carry = 0;

            while !(p1.is_null() && p2.is_null()) {
                let number = if !p1.is_null() && !p2.is_null() {
                    let number = (*p1).val + (*p2).val + carry;
                    p1 = Solution::make_ptr(&(*p1).next);
                    p2 = Solution::make_ptr(&(*p2).next);
                    number
                } else if !p1.is_null() {
                    let number = (*p1).val + carry;
                    p1 = Solution::make_ptr(&(*p1).next);
                    number
                } else {
                    let number = (*p2).val + carry;
                    p2 = Solution::make_ptr(&(*p2).next);
                    number
                };
                (*last).next = Some(Box::new(ListNode::new(number % 10)));
                carry = number / 10;
                last = Solution::make_ptr(&(*last).next);
            }

            if carry != 0 {
                (*last).next = Some(Box::new(ListNode::new(carry)));
            }

            head.next
        }
    }
}

struct Solution {}

use leetcode_solution::{ListNode, make_list, print_list};

fn main() {
    print_list(Solution::add_two_numbers(make_list(vec![2, 4, 3]), make_list(vec![5, 6, 4]))); // [7,0,8]
}