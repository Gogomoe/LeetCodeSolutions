use std::collections::BinaryHeap;
use std::cmp::min;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len() - k as usize + 1);
        for num in nums.iter() {
            if heap.len() < heap.capacity() {
                heap.push(*num);
            } else {
                let min_val = min(*num, heap.pop().unwrap());
                heap.push(min_val);
            }
        }
        return heap.pop().unwrap();
    }
}

struct Solution {}

fn main() {}