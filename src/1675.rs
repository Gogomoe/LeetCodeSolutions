use std::collections::HashMap;
use std::ptr;
use std::mem::swap;
use std::cmp::{max, min};
use std::hash::Hash;

struct PriorityQueue<V> {
    keys: Vec<Option<i32>>,
    values: Vec<Option<V>>,
    size: usize,
}

impl<V> PriorityQueue<V> {
    fn new() -> PriorityQueue<V> {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        keys.push(None);
        values.push(None);
        PriorityQueue {
            keys,
            values,
            size: 0,
        }
    }

    fn pop(&mut self) -> (i32, V) {
        if self.size == 0 {
            panic!("PriorityQueue empty");
        }
        self.swap(1, self.size);
        let k = self.keys.remove(self.size).unwrap();
        let v = self.values.remove(self.size).unwrap();
        self.size -= 1;
        self.sink(1);

        (k, v)
    }

    fn push(&mut self, key: i32, value: V) {
        self.size += 1;
        self.keys.push(Some(key));
        self.values.push(Some(value));
        self.swim(self.size);
    }

    fn swim(&mut self, mut index: usize) {
        while index > 1 && self.keys[index] < self.keys[index / 2] {
            self.swap(index, index / 2);
            index /= 2;
        }
    }

    fn sink(&mut self, mut index: usize) {
        while index * 2 <= self.size {
            let mut j = index * 2;
            if j + 1 <= self.size && self.keys[j + 1] < self.keys[j] {
                j += 1;
            }
            if self.keys[index] > self.keys[j] {
                self.swap(index, j);
            } else {
                break;
            }
            index = j;
        }
    }

    pub fn swap(&mut self, ia: usize, ib: usize) {
        unsafe {
            let ka: *mut Option<i32> = &mut self.keys[ia];
            let kb: *mut Option<i32> = &mut self.keys[ib];
            ptr::swap(ka, kb);

            let va: *mut Option<V> = &mut self.values[ia];
            let vb: *mut Option<V> = &mut self.values[ib];
            ptr::swap(va, vb);
        }
    }
}

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        let mut max_value = 0;
        for it in nums.iter() {
            let mut x = it.clone();
            while x % 2 == 0 {
                x = x / 2
            }
            pq.push(x, it.clone());
            max_value = max(max_value, x);
        }
        let mut result = i32::max_value();
        while pq.size == nums.len() {
            let (k, v) = pq.pop();
            result = min(result, max_value - k);
            if k % 2 == 1 || k < v {
                pq.push(k * 2, v);
                max_value = max(max_value, k * 2);
            }
        }
        result
    }
}

struct Solution {}

fn main() {
    let res = Solution::minimum_deviation(Vec::from([2, 10, 8]));
    println!("{}", res);
}