use std::cmp::{max, min};
use std::iter;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        let mut min_value = i32::MAX;
        let mut max_value = i32::MIN;
        for x in nums.iter() {
            min_value = min(min_value, *x);
            max_value = max(max_value, *x);
        }
        let gap = ((max_value - min_value) as f64 / (nums.len() as i32 - 1) as f64).ceil() as i32;
        let mut arr: Vec<(i32, i32)> = iter::repeat((i32::MAX, i32::MIN)).take(nums.len() - 1).collect();
        for x in nums.iter() {
            if *x == min_value || *x == max_value {
                continue;
            }
            let bucket = ((*x - min_value) / gap) as usize;
            let (a, b) = arr[bucket];
            arr[bucket] = (min(a, *x), max(b, *x));
        }

        let mut prev = min_value;
        let mut result = i32::MIN;
        for i in 0..(nums.len() - 1) {
            if arr[i] == (i32::MAX, i32::MIN) { continue; }
            let (a, b) = arr[i];
            result = max(result, a - prev);
            prev = b;
        }
        result = max(result, max_value - prev);

        return result;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::maximum_gap(vec![3, 6, 9, 1])); // 3
    println!("{}", Solution::maximum_gap(vec![10])); // 0
}