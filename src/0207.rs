use std::collections::{HashSet, VecDeque};

struct Node {
    id: usize,
    outs: HashSet<usize>,
    ins: HashSet<usize>,
}

impl Node {
    fn new(id: usize) -> Node {
        Node {
            id,
            outs: HashSet::new(),
            ins: HashSet::new(),
        }
    }
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut nodes = Vec::new();
        for i in 0..num_courses {
            nodes.push(Node::new(i as usize));
        }
        for a in prerequisites.iter() {
            nodes[a[0] as usize].ins.insert(a[1] as usize);
            nodes[a[1] as usize].outs.insert(a[0] as usize);
        }
        let mut queue = VecDeque::new();
        for node in nodes.iter() {
            if node.ins.is_empty() {
                queue.push_back(node.id)
            }
        }

        while !queue.is_empty() {
            let id = queue.pop_front().unwrap();
            let outs = nodes[id].outs.clone();
            for out in outs.iter() {
                nodes[*out].ins.remove(&id);
                if nodes[*out].ins.is_empty() {
                    queue.push_back(*out)
                }
            }
        }

        for node in nodes.iter() {
            if !node.ins.is_empty() {
                return false;
            }
        }

        true
    }
}

struct Solution {}

fn main() {}