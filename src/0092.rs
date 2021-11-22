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

fn main() {
    println!("{:?}", Solution::reverse_between(make_list(vec![1, 2, 3, 4, 5]), 2, 4));
    // [1,4,3,2,5]
    println!("{:?}", Solution::reverse_between(make_list(vec![1, 2, 3, 4, 5]), 2, 5));
    // [1,5,4,3,2]
    println!("{:?}", Solution::reverse_between(make_list(vec![1, 2, 3, 4, 5]), 1, 4));
    // [4,3,2,1,5]
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