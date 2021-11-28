// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn make_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut list = None;
    for value in vec.into_iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = list;
        list = Some(node);
    }
    return list;
}

pub fn print_list(node: Option<Box<ListNode>>) {
    print_list_ref(&node);
}

pub fn print_list_ref(mut node: &Option<Box<ListNode>>) {
    let mut list = Vec::new();
    while node.is_some() {
        list.push(node.as_ref().unwrap().val.to_string());
        node = &node.as_ref().unwrap().next;
    }
    println!("[{}]", list.join(","));
}
