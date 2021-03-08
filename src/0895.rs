use std::collections::{HashMap, VecDeque};
use std::cmp::max;

struct FreqStack {
    cnt: HashMap<i32, usize>,
    stacks: Vec<VecDeque<i32>>,
    max_cnt: usize,
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            cnt: HashMap::new(),
            stacks: Vec::new(),
            max_cnt: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let count = self.cnt.entry(val).or_insert(0);
        *count += 1;
        self.max_cnt = max(self.max_cnt, *count);
        if self.stacks.len() < *count {
            self.stacks.push(VecDeque::new());
        }
        self.stacks[*count - 1].push_back(val);
    }

    fn pop(&mut self) -> i32 {
        while self.stacks[self.max_cnt - 1].is_empty() {
            self.max_cnt -= 1;
        }
        let val = self.stacks[self.max_cnt - 1].pop_back().unwrap();
        *self.cnt.get_mut(&val).unwrap() -= 1;
        val
    }
}

fn main() {}