use std::collections::HashSet;
use std::cmp::min;

impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let mut prev = HashSet::new();
        let mut next = HashSet::new();
        let mut min_val = i32::max_value();

        for num in arr.iter().rev() {
            for it in prev.iter() {
                let new_val = num & it;
                next.insert(new_val);
                min_val = min(min_val, (new_val - target).abs());
            }
            next.insert(*num);
            min_val = min(min_val, (num - target).abs());

            prev = next;
            next = HashSet::new();
        }

        min_val
    }
}

struct Solution {}

fn main() {}