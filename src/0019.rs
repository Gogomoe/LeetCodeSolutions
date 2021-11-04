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

fn main() {
    let node5 = ListNode::new(5);
    let mut node4 = ListNode::new(4);
    let mut node3 = ListNode::new(3);
    let mut node2 = ListNode::new(2);
    let mut node1 = ListNode::new(1);

    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));

    println!("{:?}", Solution::remove_nth_from_end(Some(Box::new(node1)), 2));
    println!("{:?}", Solution::remove_nth_from_end(Some(Box::new(ListNode::new(1))), 1));
}