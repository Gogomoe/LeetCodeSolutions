use std::ptr::null_mut;

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

fn main() {
    let mut l2 = Box::new(ListNode::new(2));
    let mut l4 = Box::new(ListNode::new(4));
    let l3 = Box::new(ListNode::new(3));
    l4.next = Some(l3);
    l2.next = Some(l4);

    let mut l5 = Box::new(ListNode::new(5));
    let mut l6 = Box::new(ListNode::new(6));
    let l4 = Box::new(ListNode::new(4));
    l6.next = Some(l4);
    l5.next = Some(l6);

    let mut res = Solution::add_two_numbers(Some(l2), Some(l5));
    while res.is_some() {
        let value = res.unwrap();
        println!("{}", value.val);
        res = value.next;
    }
}