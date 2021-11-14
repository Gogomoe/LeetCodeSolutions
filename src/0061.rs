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

fn main() {
    println!("{:?}", Solution::rotate_right(make_list(vec![1, 2, 3, 4, 5]), 2)); // [4,5,1,2,3]
    println!("{:?}", Solution::rotate_right(make_list(vec![0, 1, 2]), 4)); // [2,0,1]
}

fn make_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut list = None;
    for value in vec.into_iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = list;
        list = Some(node);
    }
    return list;
}